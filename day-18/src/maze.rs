use crate::a_star::{search, Searchable};
use crate::grid::{Grid, Point};

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

enum Node {
    Empty,
    Door(String),
    Key(String),
}

pub struct Maze {
    entrances: Vec<Point>,
    nodes: Grid<Node>,
    doors: HashMap<String, Point>,
    keys: HashMap<String, Point>,
    x_max: i32,
    y_max: i32,
    quadrants: [Point; 4],
}

impl Maze {
    fn new(
        entrances: Vec<Point>,
        nodes: Grid<Node>,
        doors: HashMap<String, Point>,
        keys: HashMap<String, Point>,
        x_max: i32,
        y_max: i32,
        quadrants: [Point; 4],
    ) -> Self {
        Maze {
            entrances,
            nodes,
            doors,
            keys,
            x_max,
            y_max,
            quadrants,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut file = fs::File::open(filename).expect("Failed to open puzzle input");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read puzzle input");
        Maze::from_string(&contents)
    }

    pub fn from_string(contents: &str) -> Self {
        let mut entrances = Vec::new();
        let mut nodes = Grid::new();
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();

        let reader = BufReader::new(contents.as_bytes());

        let mut x = 0;
        let mut y = 0;
        let mut x_max = 0;
        let mut y_max = 0;

        for line in reader.lines() {
            let line = line.expect("Failed to read line from file contents");
            y_max = y_max.max(y);

            for c in line.trim().chars() {
                x_max = x_max.max(x);

                let point = Point::new(x, y);
                x += 1;

                if c == '#' {
                    continue;
                }

                match c {
                    '.' => nodes.set(point, Node::Empty),
                    '@' => {
                        entrances.push(point);
                        nodes.set(point, Node::Empty);
                    }
                    object => {
                        if object.is_uppercase() {
                            let door = object.to_string();
                            doors.insert(door.clone(), point);
                            nodes.set(point, Node::Door(door));
                        } else {
                            let key = object.to_uppercase().to_string();
                            keys.insert(key.clone(), point);
                            nodes.set(point, Node::Key(key));
                        }
                    }
                }
            }

            x = 0;
            y += 1;
        }

        let quadrants = [
            Point::new(x_max / 2, y_max / 2),
            Point::new(x_max, y_max / 2),
            Point::new(x_max / 2, y_max),
            Point::new(x_max, y_max),
        ];

        Maze::new(entrances, nodes, doors, keys, x_max, y_max, quadrants)
    }

    #[allow(dead_code)]
    pub fn to_string(
        &self,
        current: &Point,
        goal: &Point,
        frontier: &HashSet<Point>,
        explored: &HashSet<Point>,
    ) -> String {
        let mut output = String::new();

        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                let point = Point::new(x, y);
                if point == *current || point == *goal {
                    output.push('█');
                } else if frontier.contains(&point) {
                    output.push('▒');
                } else if explored.contains(&point) {
                    output.push('▓');
                } else {
                    let pixel = match self.nodes.get(&point) {
                        Some(Node::Empty) => ' ',
                        Some(Node::Door(door)) => door.chars().nth(0).unwrap(),
                        Some(Node::Key(key)) => key.to_lowercase().chars().nth(0).unwrap(),
                        None => '░',
                    };
                    output.push(pixel);
                }
            }
            output.push('\n');
        }

        output
    }

    pub fn get_entrances(&self) -> &Vec<Point> {
        &self.entrances
    }

    pub fn get_key(&self, key: &str) -> Option<&Point> {
        self.keys.get(key)
    }

    pub fn get_keys(&self) -> Vec<String> {
        let mut keys = self.keys.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        keys
    }

    pub fn get_quadrant(&self, point: &Point) -> Point {
        *self
            .quadrants
            .iter()
            .find(|quadrant| point.x <= quadrant.x && point.y <= quadrant.y)
            .expect("Failed to get quadrant for point")
    }

    pub fn in_same_quadrant(&self, a: &Point, b: &Point) -> bool {
        self.get_quadrant(a) == self.get_quadrant(b)
    }

    pub fn get_path(&self, start: &Point, key: &str) -> Option<Vec<Point>> {
        let key_point = self.get_key(key).expect("Failed to get location of key");
        search(start, key_point, &self.nodes, |a, b| self.heuristic(a, b))
    }

    pub fn get_path_from_entrance(&self, key: &str) -> Option<Vec<Point>> {
        for entrance in self.entrances.iter() {
            let path = self.get_path(entrance, key);
            if path.is_some() {
                return path;
            }
        }

        None
    }

    pub fn get_path_between_keys(&self, start_key: &str, end_key: &str) -> Option<Vec<Point>> {
        let start_point = self
            .get_key(start_key)
            .expect("Failed to get location of start key");
        self.get_path(start_point, end_key)
    }

    pub fn get_doors_on_path(&self, path: &Vec<Point>) -> Vec<String> {
        path.iter()
            .filter_map(|point| match self.nodes.get(point) {
                Some(Node::Door(door)) => Some(door),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn heuristic(&self, current: &Point, goal: &Point) -> usize {
        let quadrant_cost = if self.in_same_quadrant(current, goal) {
            0
        } else {
            10000
        };
        self.nodes.distance(current, goal).pow(2) + quadrant_cost
    }
}

impl Searchable<Point> for Maze {
    fn distance(&self, a: &Point, b: &Point) -> usize {
        self.nodes.distance(a, b)
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        self.nodes.get_neighbors(point)
    }

    // fn debug(
    //     &self,
    //     current: &Point,
    //     goal: &Point,
    //     frontier: &HashSet<Point>,
    //     explored: &HashSet<Point>,
    // ) {
    //     let output = self.to_string(current, goal, frontier, explored);
    //     println!("{}", output);
    //     thread::sleep(Duration::from_millis(10));
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "#########
                     #b.A.@.a#
                     #########";
        let maze = Maze::from_string(input);

        assert_eq!(maze.nodes.len(), 7);
        assert_eq!(maze.entrances[0], Point::new(5, 1));
        assert_eq!(maze.doors.len(), 1);
        assert_eq!(maze.doors.get("A"), Some(&Point::new(3, 1)));
        assert_eq!(maze.keys.len(), 2);
        assert_eq!(maze.keys.get("A"), Some(&Point::new(7, 1)));
        assert_eq!(maze.keys.get("B"), Some(&Point::new(1, 1)));
    }
}
