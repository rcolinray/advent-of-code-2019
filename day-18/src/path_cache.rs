use crate::maze::Maze;
use crate::point::Point;

use std::collections::HashMap;

pub const ENTRANCE: &str = "@";

pub struct Path {
    points: Vec<Point>,
    doors: Vec<String>,
}

impl Path {
    pub fn new(points: Vec<Point>, doors: Vec<String>) -> Self {
        Path { points, doors }
    }

    pub fn len(&self) -> usize {
        self.points.len() - 1
    }

    pub fn is_unlocked(&self, keys: &Vec<String>) -> bool {
        self.doors.iter().all(|door| keys.contains(&door))
    }
}

pub struct PathCache {
    paths: HashMap<(String, String), Path>,
}

impl PathCache {
    pub fn new(maze: &Maze) -> Self {
        let all_keys = maze.get_keys();
        let mut paths = HashMap::new();

        for key in all_keys.iter() {
            let points = maze
                .get_path_from_entrance(&key)
                .expect("Failed to get path to initial key");
            let doors = maze.get_doors_on_path(&points);
            let path = Path::new(points, doors);
            paths.insert((ENTRANCE.to_owned(), key.clone()), path);
        }

        for start_key in all_keys.iter() {
            for end_key in all_keys.iter() {
                if start_key == end_key
                    || paths.get(&(start_key.clone(), end_key.clone())).is_some()
                {
                    continue;
                }

                if let Some(points) = maze.get_path_between_keys(start_key, end_key) {
                    let doors = maze.get_doors_on_path(&points);
                    let reverse_points = points.iter().rev().copied().collect::<Vec<_>>();

                    let path = Path::new(points, doors.clone());
                    let rev_path = Path::new(reverse_points, doors);

                    paths.insert((start_key.clone(), end_key.clone()), path);
                    paths.insert((end_key.clone(), start_key.clone()), rev_path);
                }
            }
        }

        PathCache { paths }
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn get(&self, start: &str, end: &str) -> Option<&Path> {
        self.paths.get(&(start.to_owned(), end.to_owned()))
    }
}
