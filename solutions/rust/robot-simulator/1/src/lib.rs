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
    position_x: i32,
    position_y: i32,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position_x: x,
            position_y: y,
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East
        };

        Robot {
            direction: new_direction,
            position_x: self.position_x,
            position_y: self.position_y,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        };

        Robot {
            direction: new_direction,
            position_x: self.position_x,
            position_y: self.position_y,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let new_position = match self.direction {
            Direction::North => (self.position_x, self.position_y + 1),
            Direction::South => (self.position_x, self.position_y - 1),
            Direction::East => (self.position_x + 1, self.position_y),
            Direction::West => (self.position_x - 1, self.position_y),
        };

        Robot {
            direction: self.direction,
            position_x: new_position.0,
            position_y: new_position.1,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            robot = match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("Invalid Instruction")
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position_x, self.position_y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
