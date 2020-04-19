pub fn some_functions() {
    let thought = create_string();
    println!("What do you think? {}", thought);

    let mut value: i8 = 10;
    increase_by_ref(&mut value);
    println!("This is the value after increase by one: {}", value);

    use_closures();
}

fn create_string() -> String {
    String::from("This is what I think")
}

fn increase_by_ref(value: &mut i8) {
    let old_value = *value;
    println!("Old value was: {}", old_value);
    *value = old_value + 1;
}

fn use_closures() {
    let increase = 
        |number: i8| -> i8 { number + 1 };

    let value = 5;
    println!("Current number is: {} and after increasing with closure is: {}", value, increase(value));

    fn decrease(number: i8) -> i8 {
        number - 1
    }

    println!("Current number is: {} and after decreasing with closure is: {}", value, decrease(value));
}