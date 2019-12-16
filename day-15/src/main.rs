mod ai;
mod droid;
mod intcode;
mod point;
mod section_map;

use ai::Ai;
use droid::{Direction, Droid, MoveResult};
use intcode::{load_program, Computer};
use point::Point;
use section_map::SectionMap;

// use std::io;

fn main() {
    // demo();
    part1();
    part2();
}

// #[allow(dead_code)]
// fn demo() {
//     let mem = load_program("./repair.intcode");
//     let mut cpu = Computer::new(&mem);
//     let mut droid = Droid::new(cpu);
//     let mut map = SectionMap::new();

//     loop {
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read input");
//         input.truncate(input.len() - 1);
//         if let Some(dir) = Direction::from_string(&input) {
//             let result = droid.try_move(dir);
//             map.set_result(dir, result);
//         }
//         println!("{}", map.to_string());
//     }
// }

fn part1() {
    let mem = load_program("./repair.intcode");
    let cpu = Computer::new(&mem);
    let mut droid = Droid::new(cpu);
    let mut map = SectionMap::new();
    let mut ai = Ai::new(&map);

    loop {
        ai.update(&mut droid, &mut map);
        println!("{}", map.to_string(ai.get_pos()));
        if ai.is_found() {
            break;
        }
    }

    let length = ai.get_path_length(&map);
    println!("part 1: {:?}", length);
}

fn part2() {
    let mem = load_program("./repair.intcode");
    let cpu = Computer::new(&mem);
    let mut droid = Droid::new(cpu);
    let mut map = SectionMap::new();
    let mut ai = Ai::new(&map);

    loop {
        ai.update(&mut droid, &mut map);
        println!("{}", map.to_string(ai.get_pos()));
        if ai.done_exploring() {
            break;
        }
    }

    let oxygen_system = ai.get_oxygen_system();
    let minutes = flood_oxygen(oxygen_system, &mut map);
    println!("part 2: {}", minutes);
}

fn flood_oxygen(oxygen_system: Point, map: &mut SectionMap) -> usize {
    let mut minutes = 0;

    let mut frontier = Vec::new();
    let neighbors = map.get_empty_neighbors(oxygen_system);
    frontier.extend(neighbors.iter());

    loop {
        let mut next_frontier = Vec::new();
        while frontier.len() > 0 {
            let current = frontier.pop().expect("Failed to get point from frontier");
            map.set_result(current, MoveResult::Oxygen);
            let new_frontier = map.get_empty_neighbors(current);
            next_frontier.extend(new_frontier.iter());
        }
        frontier = next_frontier;
        println!("{}", map.to_string(Point::at(0, 0)));
        minutes += 1;

        if frontier.len() == 0 {
            break;
        }
    }

    minutes
}
