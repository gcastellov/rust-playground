pub fn find_values() {
    get_value_at(1);
    get_value_at(4);
    get_value_at(5);
}

fn get_value_at(index: usize) {
    let numbers: [u8;5] = [5,6,7,8,9];
    let score = numbers.get(index);

    if let Some(x) = score {
        println!("Found {} at position {}", x, index)
    }
    else {
        println!("Nothing found at position {} because lenght of {}", index, numbers.len())
    }
}