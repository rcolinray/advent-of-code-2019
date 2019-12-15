use std::collections::HashMap;

use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    pub fn from_int(n: i64) -> Tile {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            bad_n => panic!("Could not create Tile from integer {}", bad_n),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Empty => ' ',  //'\u{2B1B}',   // black
            Self::Wall => 'W',   //'\u{1F7E5}',   // red
            Self::Block => 'B',  //'\u{1F7E7}',  // orange
            Self::Paddle => 'P', //'\u{1F7E8}', // yellow
            Self::Ball => 'O',   //'\u{26AA}',    // white circle
        }
    }
}

pub struct Screen {
    tiles: HashMap<Point, Tile>,
    x_max: i64,
    y_max: i64,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            tiles: HashMap::new(),
            x_max: 0,
            y_max: 0,
        }
    }

    pub fn set(&mut self, point: Point, n: i64) {
        let tile = Tile::from_int(n);
        self.x_max = self.x_max.max(point.x);
        self.y_max = self.y_max.max(point.y);
        self.tiles.insert(point, tile);
    }

    pub fn num_blocks(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_, &t)| match t {
                Tile::Block => true,
                _ => false,
            })
            .count()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                let tile = self.tiles.get(&Point::at(x, y)).unwrap_or(&Tile::Empty);
                output.push(tile.to_char());
            }
            output.push('\n');
        }

        output
    }
}
