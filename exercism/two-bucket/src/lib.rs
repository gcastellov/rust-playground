#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

struct Cube {
    bucket_type: Bucket,
    capacity: u8,
    filled: u8,
    full_when_moved: bool,
}

impl Cube {
    fn new(bucket_type: Bucket, capacity: u8) -> Self {
        Cube {
            bucket_type,
            capacity,
            filled: 0,
            full_when_moved: false,
        }
    }

    fn move_from(&mut self, cube: &mut Cube) {
        let available = self.capacity - self.filled;
        let taken_from_other_cube = if available <= cube.filled {
            available
        } else {
            cube.filled
        };
        self.filled += taken_from_other_cube;
        cube.filled -= taken_from_other_cube;
        self.full_when_moved = self.is_full();
    }

    fn empty(&mut self) {
        self.filled = 0;
        self.full_when_moved = false;
    }

    fn fill(&mut self) {
        self.filled = self.capacity;
        self.full_when_moved = false;
    }

    fn is_empty(&self) -> bool {
        self.filled == 0
    }

    fn is_full(&self) -> bool {
        self.capacity - self.filled == 0
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut one = Cube::new(Bucket::One, capacity_1);
    let mut two = Cube::new(Bucket::Two, capacity_2);
    let moves = pour(&mut one, &mut two, goal, *start_bucket)?;

    if one.filled == goal {
        Some(BucketStats {
            moves: moves.len() as u8,
            goal_bucket: one.bucket_type,
            other_bucket: two.filled,
        })
    } else {
        Some(BucketStats {
            moves: moves.len() as u8,
            goal_bucket: two.bucket_type,
            other_bucket: one.filled,
        })
    }
}

fn pour(
    mut one: &mut Cube,
    mut two: &mut Cube,
    goal: u8,
    starting: Bucket,
) -> Option<Vec<(u8, u8)>> {
    let mut moves: Vec<(u8, u8)> = Vec::default();
    let mut turn: Bucket = starting;

    while one.filled != goal && two.filled != goal {
        if turn == Bucket::One && one.is_empty() {
            one.fill();
            if two.capacity == goal {
                turn = Bucket::Two;
            }
        } else if turn == Bucket::One && one.full_when_moved {
            one.empty();
            turn = Bucket::Two;
        } else if turn == Bucket::One && !one.is_empty() && !two.is_full() {
            two.move_from(&mut one);
            if !one.is_empty() {
                turn = Bucket::Two;
            }
        } else if turn == Bucket::Two && two.is_empty() {
            two.fill();
            if one.capacity == goal {
                turn = Bucket::One;
            }
        } else if turn == Bucket::Two && two.full_when_moved {
            two.empty();
            turn = Bucket::One;
        } else if turn == Bucket::Two && !two.is_empty() && !one.is_full() {
            one.move_from(&mut two);
            if !two.is_empty() {
                turn = Bucket::One;
            }
        } else {
            return None;
        }

        moves.push((one.filled, two.filled));
    }

    Some(moves)
}
