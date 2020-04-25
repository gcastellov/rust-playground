pub fn verse(n: u32) -> String {
    let current_bottles: String = format_bottles(n);
    let mut remaining_bottles: String = String::from("no more bottles");
    let mut complement: String = String::from("it");

    if n > 0 {
        if n > 1 {
            remaining_bottles = format_bottles(n-1);
            complement = String::from("one");
        }

        format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", 
            current_bottles, current_bottles, complement, remaining_bottles)
    }
    else {
        format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
}

pub fn sing(start: u32, end: u32) -> String {

    (end..=start)
        .rev()
        .map(|i|{
            let mut verse = verse(i);
            if i != end { verse += "\n" }
            verse
        })
        .collect()
}

fn format_bottles(number: u32) -> String {
    let mut output: String = format!("{} bottle", number);
    if number > 1 {
        output.push('s');
    }

    output
}
