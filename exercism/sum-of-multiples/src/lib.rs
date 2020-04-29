use itertools::Itertools;

const USE_FUNCTIONAL:bool = true;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if USE_FUNCTIONAL {
        return sum_as_functional(limit, factors);
    }

    sum(limit, factors)
}

fn sum_as_functional(limit: u32, factors: &[u32]) -> u32 {    
    
    factors
        .iter()
        .map(|factor| 
            (1..limit)
                .into_iter()
                .filter(move |num|factor > &0 && num % factor == 0)
            )
        .flatten()
        .collect::<Vec<u32>>()
        .into_iter()
        .unique()
        .sum()
}

fn sum(limit: u32, factors: &[u32]) -> u32 {    
    
    let mut multiples = Vec::<u32>::new();
    let mut sum: u32 = 0;
    
    for factor in factors {
        for number in 1..limit {
            if !multiples.contains(&number) && factor > &0 && number % factor == 0 {
                multiples.push(number);
                sum += number;
            }
        }
    }

    sum
}
