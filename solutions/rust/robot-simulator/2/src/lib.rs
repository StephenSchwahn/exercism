// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::East => Self { direction: Direction::South, .. self },
            Direction::South => Self { direction: Direction::West, .. self },
            Direction::West =>  Self { direction: Direction::North, ..self },
            Direction::North =>  Self { direction: Direction::East, .. self },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::East => Self { direction: Direction::North, .. self },
            Direction::South => Self { direction: Direction::East, .. self },
            Direction::West =>  Self { direction: Direction::South, .. self },
            Direction::North =>  Self { direction: Direction::West, .. self },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self { y: self.y + 1, .. self },
            Direction::South => Self { y: self.y - 1, .. self },
            Direction::East => Self { x: self.x + 1, .. self },
            Direction::West => Self { x: self.x - 1, .. self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instruction| {
            match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("Invalid Instruction")
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
