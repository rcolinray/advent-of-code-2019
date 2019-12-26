mod grid;
mod maze;
mod path_cache;
mod point;
mod search;

use std::env::args;

use maze::{Maze, RecursiveMaze};

fn main() {
    let filename = args().nth(1).unwrap_or("./input.txt".to_owned());
    part1(&filename);
    part2(&filename);
}

fn part1(filename: &str) {
    let maze = Maze::from_file(filename);
    let len = maze.find_shortest_path_len().unwrap();
    println!("part 1: {:?}", len);
}

fn part2(filename: &str) {
    let maze = RecursiveMaze::from_file(filename);
    let len = maze.find_shortest_path_len().unwrap();
    println!("part 2: {:?}", len);
}
