use crate::intcode::Computer;
use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_string(string: &str) -> Option<Direction> {
        match string.trim().to_uppercase().as_ref() {
            "N" => Some(Direction::North),
            "S" => Some(Direction::South),
            "E" => Some(Direction::East),
            "W" => Some(Direction::West),
            _ => None,
        }
    }

    pub fn from_data(data: i64) -> Self {
        match data {
            1 => Self::North,
            2 => Self::South,
            3 => Self::East,
            4 => Self::West,
            _ => panic!("Could not create direction from integer"),
        }
    }

    pub fn to_data(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::East => 3,
            Self::West => 4,
        }
    }

    pub fn to_vector(self) -> Point {
        match self {
            Self::North => Point::at(0, 1),
            Self::South => Point::at(0, -1),
            Self::East => Point::at(1, 0),
            Self::West => Point::at(-1, 0),
        }
    }

    pub fn from_vector(point: &Point) -> Self {
        match point {
            &Point { x: 0, y: 1 } => Self::North,
            &Point { x: 0, y: -1 } => Self::South,
            &Point { x: 1, y: 0 } => Self::East,
            &Point { x: -1, y: 0 } => Self::West,
            bad_point => panic!("Could not create direction from vector: {:?}", bad_point),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MoveResult {
    Wall,
    Empty,
    Oxygen,
}

impl MoveResult {
    pub fn from_data(data: i64) -> Self {
        match data {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::Oxygen,
            _ => panic!("bad data"),
        }
    }

    pub fn is_oxygen_system(&self) -> bool {
        match self {
            Self::Oxygen => true,
            _ => false,
        }
    }

    pub fn is_wall(&self) -> bool {
        match self {
            Self::Wall => true,
            _ => false,
        }
    }
}

pub struct Droid {
    cpu: Computer,
}

impl Droid {
    pub fn new(cpu: Computer) -> Self {
        Droid { cpu }
    }

    pub fn try_move(&mut self, direction: Direction) -> MoveResult {
        self.cpu.set_input(direction.to_data());
        self.cpu.run();
        let output = self
            .cpu
            .get_output()
            .expect("Failed to get output from CPU");
        MoveResult::from_data(output)
    }
}
