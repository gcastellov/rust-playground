pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::default();
    let mut array: Vec<u64> = (2..=upper_bound).map(|num| num).collect();

    for index in 0..array.len() {
        let num = array[index];
        if num > 0 && num % num == 0 {
            primes.push(num);
            let mut current_index = index;
            while current_index < array.len() {
                array[current_index] = 0;
                current_index += num as usize;
            }
        }
    }

    primes
}
