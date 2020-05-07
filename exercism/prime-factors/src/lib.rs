pub fn factors(n: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    let mut current: u64 = n;
    let mut i = 2;

    while current > 1 {
        if current.rem_euclid(i) > 0 {
            i += 1;
        }
        else {
            current = current.div_euclid(i);
            output.push(i);
        }
    }

    output
}
