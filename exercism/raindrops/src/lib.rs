pub fn raindrops(n: u32) -> String {
    
    let mut drops: String = (1..=n)
        .filter(|number|n % number == 0)
        .map(|number|match_drop(number))
        .filter(|option|option.is_some())
        .map(|option|option.unwrap())
        .collect();

    if drops.is_empty() {
        drops = n.to_string();
    }

    drops
}

fn match_drop(n: u32) -> Option<String> {
    match n {
        3 => Some("Pling".to_string()),
        5 => Some("Plang".to_string()),
        7 => Some("Plong".to_string()),
        _ => None
    }
}