use crate::droid::{Direction, MoveResult};
use crate::point::Point;

use std::collections::HashMap;

pub struct SectionMap {
    map: HashMap<Point, MoveResult>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl SectionMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Point::at(0, 0), MoveResult::Empty);
        SectionMap {
            map,
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }

    pub fn set_result(&mut self, point: Point, result: MoveResult) {
        self.x_min = self.x_min.min(point.x);
        self.x_max = self.x_max.max(point.x);
        self.y_min = self.y_min.min(point.y);
        self.y_max = self.y_max.max(point.y);
        self.map.insert(point, result);
    }

    pub fn get_result(&self, point: &Point) -> Option<MoveResult> {
        self.map.get(point).cloned()
    }

    pub fn is_empty(&self, point: &Point) -> bool {
        match self.get_result(point) {
            Some(MoveResult::Empty) => true,
            _ => false,
        }
    }

    pub fn get_neighbors(&self, point: Point) -> Vec<Point> {
        vec![
            point + Direction::North.to_vector(),
            point + Direction::South.to_vector(),
            point + Direction::East.to_vector(),
            point + Direction::West.to_vector(),
        ]
        .iter()
        .cloned()
        .collect::<Vec<_>>()
    }

    pub fn get_empty_neighbors(&self, point: Point) -> Vec<Point> {
        self.get_neighbors(point)
            .iter()
            .filter(|point| self.is_empty(point))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn to_string(&self, pos: Point) -> String {
        let mut output = String::new();

        for y in (self.y_min - 1..=self.y_max + 1).rev() {
            for x in self.x_min - 1..=self.x_max + 1 {
                let point = Point::at(x, y);
                let pixel = if point == pos {
                    'D'
                } else {
                    match self.get_result(&Point::at(x, y)) {
                        Some(MoveResult::Wall) => '#',
                        Some(MoveResult::Empty) => '.',
                        Some(MoveResult::Oxygen) => 'O',
                        _ => ' ',
                    }
                };
                output.push(pixel);
            }

            output.push('\n');
        }

        output
    }
}
