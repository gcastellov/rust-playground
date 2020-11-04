use std::collections::HashMap;

const DROP: &str = "DROP";
const DUP: &str = "DUP";
const SWAP: &str = "SWAP";
const OVER: &str = "OVER";
const ADD: &str = "+";
const SUB: &str = "-";
const TIMES: &str = "*";
const DIV: &str = "/";
const COLON: &str = ":";
const SEMICOLON: &str = ";";
const ZERO: &str = "0";

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, Clone)]
enum Operator {
    Stack(String),
    Arithmetic(String)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, Clone)]
struct Item {
    value: String,
    operator: Option<Operator>
}

struct Entry {
    items: Vec<Item>
}

pub struct Forth {
    values: Vec<Value>,
    aliases: HashMap<String, Vec<Item>>
}

impl Operator {
    fn new(input: String) -> Option<Self> {
        match input.as_str() {
            DROP | DUP | SWAP | OVER => Some(Operator::Stack(input)),
            ADD | SUB | TIMES | DIV => Some(Operator::Arithmetic(input)),
            _ => None
        }
    }
}

impl Item {
    fn new(input:&str) -> Self {
        let data = input.to_uppercase();
        Item { value: data.clone(), operator: Operator::new(data) }
    }

    fn as_integer(&self) -> Option<Value> {
        let parsed_value = self.value.parse();
        if parsed_value.is_err() {
            return None;
        }

        parsed_value.ok()
    }

    fn is_operator(&self) -> bool {
        self.operator.is_some()
    }

    fn is_stack_operator(&self) -> bool {
        match self.operator {
            Some(Operator::Stack(_))  => true,
            _ => false
        }
    }
    
    fn is_arithmetic_operator(&self) -> bool {
        match self.operator {
            Some(Operator::Arithmetic(_))  => true,
            _ => false
        }
    }

    fn is_starting_alias(&self) -> bool {
        self.value.as_str() == COLON
    }

    fn is_ending_alias(&self) -> bool {
        self.value.as_str() == SEMICOLON
    }
}

impl Entry {
    fn new(input: &str) -> Self {
        Entry { items: 
            input
                .split_whitespace()
                .map(|i|Item::new(i))
                .collect()
        }
    }

    fn is_defining_alias(&self) -> bool {
        self.items.first().unwrap().is_starting_alias() && self.items.last().unwrap().is_ending_alias()
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth { values: Vec::default(), aliases: HashMap::default() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.values.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut result: ForthResult = Ok(());
        let entries: Vec<Entry> = Forth::split_entries(input)
            .iter()
            .map(|chunk|Entry::new(chunk))
            .collect();

        for entry in entries {
            if entry.is_defining_alias() {
                result = self.define_alias(&entry);
            } else {
                result = self.eval_items(&entry);
            }

            if result.is_err() {
                break;
            }
        }

        result
    }

    fn define_alias(&mut self, entry: &Entry) -> ForthResult {
        let mut items = entry.items.clone();        
        items.remove(0);
        items.pop();

        if items.iter().all(|item|item.as_integer().is_some()) {
            return Err(Error::InvalidWord);
        }

        if let Some((first, values)) = items.split_first() {
            if let Some((first_value, next_values)) = values.split_first() {
                if let Some(mut alias) = self.aliases.remove(&first_value.value) {
                    alias.append(&mut Vec::from(next_values));
                    self.aliases.insert(first.value.clone(), alias);
                } else {
                    self.aliases.insert(first.value.clone(), Vec::from(values));
                }
            }
        }       

        Ok(())
    }

    fn eval_items(&mut self, entry: &Entry) -> ForthResult {
        let mut items = entry.items.clone();
        if !self.aliases.is_empty() {
            let keywords: Vec<String> = self.aliases.iter().map(|(keyword, _)|keyword.clone()).collect();
            let mut aggregate: usize = 0;
            
            let positions: Vec<(String, usize)> = items
                .iter()
                .enumerate()
                .filter_map(|(index, item)|{
                    let value = item.value.clone();
                    if keywords.contains(&value) {
                        Some((value, index))
                    } else {
                        None
                    }
                })
                .collect();

            for (keyword, index) in positions {
                let i = index + aggregate;
                if let Some(values) = self.aliases.get(&keyword) {
                    items.splice(i..=i, values.iter().cloned());
                    aggregate = aggregate + values.len() - 1;
                }
            }
        }

        if items.iter().any(|item|!item.is_operator() && item.as_integer().is_none()) {
            if items.first().unwrap().as_integer().is_some() {
                return Err(Error::UnknownWord);
            } else {
                return Err(Error::InvalidWord);
            }
        }
        
        let last = items.last().unwrap();
        if last.is_operator() && self.values.is_empty() {
            if items.len() < 3 && last.value != DUP && last.value != DROP {
                return Err(Error::StackUnderflow);
            } else if items.len() == 1 && (last.value == DUP || last.value == DROP) {
                return Err(Error::StackUnderflow);
            }
        }

        if items.iter().all(|item|!item.is_operator()) {
            self.values = items.iter().map(|item|item.as_integer().unwrap()).collect();
            return Ok(());
        }

        let operator_indexes: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, item)|item.is_operator())
            .map(|(index, _)|index)
            .collect();

        let chunks: Vec<&[Item]> = operator_indexes
            .iter()
            .enumerate()
            .map(|(index, operator_index)| {
                if index > 0 {
                    let last_index = operator_indexes.get(index-1).unwrap();
                    &items[*last_index+1..=*operator_index]
                } else {
                    &items[0..=*operator_index]
                }
            })
            .collect();

        if chunks.iter().any(|chunk|chunk.last().unwrap().is_arithmetic_operator()) {
            let result = self.calc_chunks(chunks);
            if result.is_err() {
                return Err(result.err().unwrap());
            } else {
                self.values = vec![result.ok().unwrap()];
                return Ok(());
            }
        } else if chunks.iter().any(|chunk|chunk.last().unwrap().is_stack_operator()) {
            self.apply_stack_operators(chunks);
        }

        Ok(())
    }

    fn calc_chunks(&self, chunks: Vec<&[Item]>) -> Result<Value, Error> {
        let has_division_by_zero = chunks
            .iter()
            .filter(|chunk|{
                let items: Vec<&str> = chunk.iter().map(|item|item.value.as_str()).collect();
                items.contains(&ZERO) && items.contains(&DIV)
            })
            .count() > 0;
    
        if has_division_by_zero {
            return Err(Error::DivisionByZero);
        }
    
        let aggregate = chunks
            .iter()
            .map(|chunk|{
                let current_operator: Option<Operator> = chunk.last().unwrap().operator.clone();
                let mut chunk_values: Vec<Value> = self.values.iter().cloned().collect();                
                &chunk[0..chunk.len()-1]
                    .iter()
                    .map(|item|item.as_integer().unwrap())
                    .for_each(|value|chunk_values.push(value));

                let result: Value = match current_operator {
                    Some(Operator::Arithmetic(ref str)) => match str.as_str() {
                        SUB => {
                            if chunk_values.len() > 1 {
                                chunk_values.iter().skip(1).fold(chunk_values[0], |a,b| a - b)
                            } else {
                                chunk_values.iter().fold(0, |a,b| a - b)
                            }
                        },
                        ADD => chunk_values.iter().fold(0, |a,b| a + b),
                        TIMES => chunk_values.iter().fold(1, |a,b| a * b),
                        DIV => chunk_values.iter().skip(1).fold(chunk_values[0], |a,b| a / b),
                        _ => 0
                    },
                    _ => 0
                };
    
                (current_operator, result)
            })
            .fold(0, |a: Value, (operator, result)| {
                if a == 0 {
                    result
                } else {
                    match operator {
                        Some(Operator::Arithmetic(str)) => match str.as_str() {
                            ADD | SUB => a + result,
                            TIMES => a * result,
                            DIV => a / result,
                            _ => a
                        },
                        _ => a
                    }
                }
            });
    
        Ok(aggregate)
    }

    fn apply_stack_operators(&mut self, chunks: Vec<&[Item]>) {   
        for chunk in chunks {
            let mut values_as_integers = self.values.clone();
            let current_operator: Option<Operator> = chunk.last().unwrap().operator.clone();

            if chunk.len() > 1 {
                let chunk_values = &chunk[0..chunk.len()-1];
                chunk_values.iter().for_each(|item|values_as_integers.push(item.as_integer().unwrap()));
            }

            match current_operator {
                Some(Operator::Stack(str)) => match str.as_str() {
                    DUP => values_as_integers.push(values_as_integers.last().unwrap().clone()),
                    DROP => { 
                        let _ = values_as_integers.pop(); 
                    },
                    SWAP => { 
                        let index = values_as_integers.len()-2;
                        let prev_to_last = values_as_integers[index];
                        values_as_integers.remove(index);
                        values_as_integers.push(prev_to_last);
                    },
                    OVER => {
                        let index = values_as_integers.len()-2;
                        let prev_to_last = values_as_integers[index];
                        values_as_integers.push(prev_to_last);
                    }
                    _ => ()
                },
                _ => ()
            }
            
            self.values = values_as_integers;
        }
    }

    fn split_entries(entry: &str) -> Vec<&str> {
        let mut previous: usize = 0;
        let mut entries: Vec<&str> = Vec::default();
    
        for (index, _) in entry.match_indices(SEMICOLON) {
            let chunk = &entry[previous..=index].trim();
            for (c_index, _) in chunk.match_indices(COLON) {
                if c_index > 0 {
                    entries.push(&chunk[0..c_index]);
                    entries.push(&chunk[c_index..]);
                } else {
                    entries.push(&chunk[0..]);
                }
            }
            previous = index +1;
        }
    
        if previous < entry.len() {
            entries.push(&entry[previous..].trim());
        }
    
        entries
    }
}