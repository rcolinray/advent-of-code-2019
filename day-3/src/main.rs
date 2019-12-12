mod lib;

use lib::{parse_puzzle_input, solve_part1, solve_part2};

fn main() {
    let (wire1, wire2) = parse_puzzle_input("./input.txt");
    let point = solve_part1(&wire1, &wire2);
    println!("part 1: {}", point.manhattan_distance());
    let steps = solve_part2(&wire1, &wire2);
    println!("part 2: {}", steps);
}
