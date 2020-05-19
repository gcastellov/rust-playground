pub mod people {

    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub last_name: String,
        pub age: u16,
        pub address: Option<Box<Address>>
    }

    #[derive(Debug)]
    pub struct Address {
        pub city: String,
        pub street: String,
        pub number: u32,
        pub zip_code: String
    }

    impl Address {
        pub fn new(city: &str, street: &str, number: u32, zip_code: &str) -> Self {
            Address {
                city: city.to_string(),
                street: street.to_string(),
                number,
                zip_code: zip_code.to_string()
            }
        }
    }

    impl Person {
        pub fn new(name: &str, last_name: &str, age: u16) -> Self {
            Person {
                name: name.to_string(), 
                last_name: last_name.to_string(), 
                age,
                address: None
            }
        }

        pub fn set_address(&mut self, city: &str, street: &str, number: u32, zip_code: &str) {
            let address = Address::new(city, street, number, zip_code);
            self.address = Some(Box::new(address));
        }
    }

}

pub mod storage {

    use crate::management::people::Person;
    use std::collections::HashMap;

    pub struct Database {
        sequence: u64,
        pub people: HashMap<u64, Person>
    }

    impl Database {
        pub fn new() -> Self {
            Database { sequence: 1, people: HashMap::new() }
        }

        pub fn add(&mut self, person: Person) {
            let current = self.sequence;
            self.sequence += 1;
            self.people.insert(current, person);
        }
    }
}