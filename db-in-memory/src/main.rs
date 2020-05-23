mod management;
use std::num::ParseIntError;
use management::storage;
use management::people::*;
use std::io;
use std::io::prelude::*;

const NEW_OPTION: &'static str = "N";
const UPDATE_OPTION: &'static str = "U";
const LIST_OPTION: &'static str = "L";
const EXIT_OPTION: &'static str = "E";

trait OrElse {
    fn or_else(&self, _else: String) -> String;
    fn is_empty_or_whitespace(&self) -> bool;
}

impl OrElse for String {    
    fn or_else(&self, _else: String) -> String { 
        let copy = &*self;
        if copy.is_empty() { _else } else { copy.to_string() }
    }

    fn is_empty_or_whitespace(&self) -> bool {
        self.trim().is_empty() 
    }
}

fn main() {

    let db = &mut storage::Database::new();

    loop {
        show_menu_action();
        let choice: String = read_line();
        
        let _choice = &*choice;
        match _choice {
            NEW_OPTION => create(db),
            UPDATE_OPTION => update(db),
            LIST_OPTION => list(db),
            _ => break
        };
    }
}

fn show_menu_action() {
    println!("");
    println!("{} -> New person", NEW_OPTION);
    println!("{} -> Update person", UPDATE_OPTION);
    println!("{} -> List all people", LIST_OPTION);
    println!("{} -> Exit", EXIT_OPTION);
    println!("");
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut choice: String = String::new();
    
    while let Some(l) = stdin.lock().lines().next() {
        choice = l.unwrap();
        break;
    }

    choice
}

fn list<'a>(db: &'a mut storage::Database) {
    for person in &db.people {
        println!("{:?}", person);
    }
}

fn create<'a>(db: &'a mut storage::Database) {
    let mut is_completed = false;
    
    while !is_completed {
        println!("Type the following to create new person:");
        let personal_info = get_personal_info();
        is_completed = !personal_info.0.is_empty_or_whitespace() 
            && !personal_info.1.is_empty_or_whitespace() 
            && personal_info.2.is_ok();

        if !is_completed {
            println!("");
            println!("Please check your inputs. All data are required!");
            println!("");
        }
        else {
            let person = Person::new(&personal_info.0, &personal_info.1, personal_info.2.unwrap());
            db.add(person);
        }
    }
}

fn update<'a>(db: &'a mut storage::Database) {

    loop {
        println!("Find people to update by:");
        println!("I -> ID");
        let choice = read_line();

        let _choice = &*choice;
        match _choice {
            "I" => { 
                update_person(db);
                break;
            },
            _ => continue
        }
    }
}

fn find_by_id<'a>(db: &'a storage::Database) -> (u64, &'a Person) {

    loop {
        println!("Type the entry ID:");
        let _id = read_line();
        let id = &_id.parse::<u64>().unwrap();

        if db.people.contains_key(id) {
            return (*id, &db.people[id]);
        }
        else {
            println!("Sorry, person not found! Try again!");
        }
    }
}

fn update_person<'a>(db: &'a mut storage::Database) {
    let entry = find_by_id(db);
    let person: &Person = &*entry.1;
    let personal_info = get_personal_info();
    let age = if personal_info.2.is_err() { person.age } else { personal_info.2.unwrap() };

    let mut copy = Person::new(
        &personal_info.0.or_else(person.name.to_owned()), 
        &personal_info.1.or_else(person.last_name.to_owned()), 
        age);

    if let Some(address) = &person.address {
        copy.set_address(&address.city, &address.street, address.number, &address.zip_code);
    }
    
    db.people.insert(entry.0, copy);
}

fn get_personal_info() -> (String, String, Result<u16, ParseIntError>) {
    println!("Name:");
    let name = read_line();
    
    println!("Surname:");
    let last_name = read_line();
    
    println!("Age:");
    let _age = read_line();
    let age = _age.parse::<u16>();

    (name, last_name, age)
}