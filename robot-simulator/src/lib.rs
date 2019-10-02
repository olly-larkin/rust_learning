// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        use Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(self) -> Self {
        use Direction::*;

        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

pub struct Robot {
    coord: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            coord: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            coord: self.coord,
            direction: self.direction.turn_right(),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            coord: self.coord,
            direction: self.direction.turn_left(),
        }
    }

    pub fn advance(self) -> Self {
        use Direction::*;

        let mut coord = self.coord;
        match self.direction {
            North => coord = (coord.0, coord.1 + 1),
            East => coord = (coord.0 + 1, coord.1),
            South => coord = (coord.0, coord.1 - 1),
            West => coord = (coord.0 - 1, coord.1),
        }

        Robot {
            coord,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            match c {
                'A' => robot = robot.advance(),
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                _ => {},
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.coord
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
