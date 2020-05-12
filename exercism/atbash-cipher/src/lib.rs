const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const SPACE: &str = " ";
const CHUNK_SIZE: usize = 5;

type Iter<'a> = Box<dyn Iterator<Item = char> + 'a>;

trait AtbashConverter<'a> {
    fn convert(&self, from: &'a str, to: &'a str) -> Iter<'a>;
    fn reverse(&self) -> String;
}

impl<'a> AtbashConverter<'a> for &'a str {    
    fn convert(&self, from: &'a str, to: &'a str) -> Iter<'a> { 
        Box::new(
            self
            .chars()
            .filter(|c|!c.is_whitespace())
            .filter_map(move |c|{
                if c.is_numeric() { 
                    return Some(c) 
                }
                if let Some(i) = from.find(c.to_ascii_lowercase()) {
                    return to.chars().nth(i)
                }
                None
            }))
    }

    fn reverse(&self) -> String {
        self
        .chars()
        .rev()
        .collect()
    }
}

pub fn encode(plain: &str) -> String {
    let reversed_alphabet: String = ALPHABET.reverse();
    plain
        .convert(ALPHABET, &reversed_alphabet)
        .enumerate()
        .fold(String::from(""),|mut a,(index, _b)| {
            let rem: usize = index.rem_euclid(CHUNK_SIZE);
            if rem == 0 && index > 1 {
                a.push_str(SPACE);
            }
            a.push(_b);
            a
        })
}

pub fn decode(cipher: &str) -> String {
    let reversed_alphabet: String = ALPHABET.reverse();
    cipher
        .convert(&reversed_alphabet, ALPHABET)
        .collect()
}