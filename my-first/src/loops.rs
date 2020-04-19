pub fn some_looping() {
    println!("Calling for range loop............");
    range_loop();

    println!("Calling inclusive range loop.........");
    inclusive_range_loop();
    
    println!("Calling while loop...........");
    while_loop();

    println!("Calling iteration loop.....");
    iteration_loop();
}

fn while_loop() {
    let mut index: i64 = 0;
    while index < 1000 {
        index += 5;
        println!("This is index: {}", index);
    }
}

fn range_loop() {
    let mut index: i32;
    for i in 1..100 {
        index = i*2;
        println!("Index is: {}", index);
    }
}

fn inclusive_range_loop() {
    let mut index: i32;
    for i in 1..=100 {
        index = i*2;
        println!("Index is: {}", index);
    }
}

fn iteration_loop() {
    let name_list = vec!["Maria", "Laura", "Tania"];
    for name in name_list.into_iter() {
        println!("The name is: {}", name);
    }
}