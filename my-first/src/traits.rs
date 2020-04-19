trait Animal {
    fn talk(&self) -> String;
    fn create(name: &'static str, age: u8) -> Self;
}

struct Human {
    name: String,
    age: u8
}

struct Dog {
    name: String,
    age: u8
}

impl Animal for Human {
    fn talk(&self) -> String {
        let mut output: String = "Hello world. My name is ".to_string();
        output += &self.name;
        output += &" and I'm ";
        output += &self.age.to_string();
        output
    }

    fn create(name: &'static str, age: u8) -> Human {
        Human { 
            name: name.to_string(), 
            age: age 
         }
    }
}

impl Animal for Dog {
    fn talk(&self) -> String {
        "Just barking around no matter my name".to_string()
    }

    fn create(name: &'static str, age: u8) -> Dog {
        Dog { 
            name: name.to_string(), 
            age: age 
         }
    }
}

pub fn some_traits() {   
    let human: Human = Animal::create("Gerard", 35);
    let dog: Dog = Animal::create("Hope", 3);

    println!("This is {} and says: {}", human.name, human.talk());
    println!("This is {} and says: {}", dog.name, dog.talk());
}