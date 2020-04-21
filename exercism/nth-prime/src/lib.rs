const USE_ON_DEMAND: bool = true;

pub fn nth(n: u32) -> u32 {
    let index = n;
    let get_primes = |i:u32| if USE_ON_DEMAND { get_primes_on_demand(i) } else { get_from_all_primes_in_range(i)};    
    get_primes(index)
}

fn get_primes_on_demand(n: u32) -> u32 {
    let index: usize = n as usize;
    let mut primes = Vec::<u32>::new();
    let mut number: u32 = 1;

    while primes.len() <= index {
        number += 1;
        if is_prime(number) {
            primes.push(number);
        }
    }

    primes[index]
}

fn get_from_all_primes_in_range(n: u32) -> u32 {
    let range: [u32; 200_000] = [0; 200_000];
    let primes = range
        .iter()
        .enumerate()
        .map(|(x, _)| x as u32)
        .filter(|num|is_prime(*num))
        .collect::<Vec<u32>>();

    let index: usize = n as usize;
    primes[index]
}

fn is_prime(n: u32) -> bool {
    let mut index: u32 = 1;
    let mut is_divisible: bool = false;

    while index <= n && !is_divisible {
        index += 1;
        is_divisible = n % index == 0;
    }
    
    is_divisible && index == n
}
