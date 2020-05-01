use std::cmp;

struct Move {
    value: u64,
    borrowed: u64
}

impl Move {
    fn aggregate(&self) -> u64 {
        self.borrowed + self.value
    }
}

impl Clone for Move {
    fn clone(&self) -> Self { 
        Move { value: self.value, borrowed: self.borrowed }
    }
}

impl Default for Move {
    fn default() -> Self { 
        Move { value: 0, borrowed : 0 }
    }
}

impl cmp::PartialEq for Move {
    fn eq(&self, other: &Self) -> bool { 
        self.borrowed == other.borrowed && self.value == other.value
    }
}

const USE_SIMPLIFIED: bool = true;

pub fn square(s: u32) -> u64 {

    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }

    if USE_SIMPLIFIED {
        return square_simplified(s);
    }

    let mut prev: Move = Default::default();
    let mut current: Move = Default::default();

    for _ in 0..s {
        current = Move { 
            value: prev.aggregate() + 1,
            borrowed: prev.aggregate()
        };

        prev = current.clone();
    }

    current.value
}

pub fn total() -> u64 {
    (1..=64u32)
        .into_iter()
        .fold(0, |a,b| a + square(b))
}

fn square_simplified(s: u32) -> u64 {
    2u64.pow(s-1)
}