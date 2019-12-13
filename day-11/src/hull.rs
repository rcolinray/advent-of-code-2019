use crate::point::Point;

use std::collections::HashMap;

pub const BLACK: i64 = 0;
pub const WHITE: i64 = 1;

pub struct Hull {
    tiles: HashMap<Point, i64>,
    max_x: i32,
    min_x: i32,
    max_y: i32,
    min_y: i32,
}

impl Hull {
    pub fn new() -> Self {
        Hull {
            tiles: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    pub fn get_color(&self, at: &Point) -> i64 {
        match self.tiles.get(at) {
            Some(&color) => color,
            None => BLACK,
        }
    }

    pub fn paint(&mut self, at: Point, color: i64) {
        self.min_x = self.min_x.min(at.x);
        self.max_x = self.max_x.max(at.x);
        self.min_y = self.min_y.min(at.y);
        self.max_y = self.max_y.max(at.y);

        self.tiles.insert(at, color);
        // println!("tiles: {:?}", self.tiles);
    }

    pub fn num_painted(&self) -> usize {
        self.tiles.len()
    }

    pub fn render(&self, location: Point, _direction: Point) -> String {
        let mut output = String::new();

        // render from top to bottom
        for y in (self.min_y - 1..=self.max_y + 1).rev() {
            // render from left to right
            for x in self.min_x - 1..=self.max_x + 1 {
                let point = Point::at(x, y);
                let pixel = if point == location {
                    '\u{1F916}'
                } else {
                    let color = self.get_color(&point);
                    match color {
                        BLACK => '\u{2B1B}',
                        WHITE => '\u{2B1C}',
                        bad_color => panic!("bad color: {:?}", bad_color),
                    }
                };
                output.push(pixel);
            }
            output.push('\n');
        }

        output
    }
}
