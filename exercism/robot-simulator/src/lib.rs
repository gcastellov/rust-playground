// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: Position { x, y },
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            _ => Direction::North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            _ => Direction::North,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        self.position = match self.direction {
            Direction::North => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::East => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::South => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            _ => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        };

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            let robot = match c {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self.advance(),
            };
            self.direction = robot.direction;
            self.position = robot.position;
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
