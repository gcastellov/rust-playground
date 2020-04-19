enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8,u8,u8)
}

pub fn some_mathing_patterns() {
    let red = Color::Red;
    match_color(red);

    let white = Color::Rgb(0,0,0);
    match_color(white);

    let other_color = Color::Rgb(0,1,250);
    match_color(other_color);

    let numbers: [u32;8] = [0,1,4,5,7,9,10,11];
    for num in numbers.iter() {
        match_number(*num);
    }
}

fn match_color(color: Color) {
    match color {
        Color::Red => println!("This is color red"),
        Color::Green => println!("This is color green"),
        Color::Blue => println!("This is color blue"),
        Color::Rgb(0, 0, 0) => println!("This is white"),
        _ => println!("This is something else")
    }
}

fn match_number(number: u32) {
    match number {
        0 => println!("This is the lower edge: {}", number),
        1 | 2 | 3 | 4 | 5 => println!("This is still a bunch: {}", number),
        6..=10 => println!("Counting with two hands: {}", number),
        _ => println!("Too large for me", )
    }
}