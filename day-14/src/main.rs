mod component;
mod puzzle;
mod reaction;
mod solver;

use std::collections::HashMap;
use std::env;

use puzzle::load_puzzle_input;
use solver::{Compound, Solver};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "./input.txt"
    };

    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let reactions = load_puzzle_input(filename);
    let mut solver = Solver::init(&reactions, 1);
    let min_ore_required = solver.solve();

    println!("---------------- PART 1 ----------------");
    println!("minimum ore required for 1 FUEL: {:?}", min_ore_required);
}

fn part2(filename: &str) {
    let reactions = load_puzzle_input(filename);
    let mut fuel_step = 1_000_000;
    let mut solver = Solver::init(&reactions, fuel_step);
    let mut prev_leftovers: Option<HashMap<Compound, usize>> = None;

    let mut fuel: usize = 0;
    let mut ore: usize = 1_000_000_000_000;
    loop {
        let (ore_required, leftovers) = solver.solve_with_leftovers();
        if ore_required <= ore {
            ore -= ore_required;
            fuel += fuel_step;
            prev_leftovers = Some(leftovers.clone());
            solver = Solver::new(&reactions, fuel_step, leftovers);
        } else if fuel_step > 1 {
            fuel_step /= 2;
            let leftovers = prev_leftovers.clone().unwrap_or(HashMap::new());
            solver = Solver::new(&reactions, fuel_step, leftovers);
        } else {
            break;
        }
    }

    println!("---------------- PART 2 ----------------");
    println!("FUEL for 1 trillion ORE: {:?}", fuel);
}
