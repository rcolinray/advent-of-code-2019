mod intcode;
mod point;

use intcode::{load_program, Computer, Memory};
use point::Point;

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem = load_program("./program.intcode");

    let mut output = String::new();
    let mut points_affected = 0;

    for y in 0..50 {
        for x in 0..50 {
            let result = if is_drone_in_beam(&mem, x, y) {
                points_affected += 1;
                '#'
            } else {
                '.'
            };
            output.push(result);
        }

        output.push('\n');
    }

    println!("{}", output);
    println!("part 1: {}", points_affected);
}

fn is_drone_in_beam(mem: &Memory, x: usize, y: usize) -> bool {
    let mut cpu = Computer::new(&mem);
    cpu.set_input(x as i64);
    cpu.set_input(y as i64);

    cpu.run();

    match cpu.get_output() {
        Some(0) => false,
        Some(1) => true,
        _ => panic!("Failed to get output from CPU"),
    }
}

fn part2() {
    let mem = load_program("./program.intcode");
    let mut beam = HashSet::new();

    let mut prev_start = 0;
    for y in 0..10000 {
        let mut prev_in_beam = false;
        let mut failed_test = false;
        let mut width = 0;
        for x in prev_start..10000 {
            let in_beam = is_drone_in_beam(&mem, x, y);
            if in_beam {
                if !prev_in_beam {
                    prev_start = x;
                    prev_in_beam = in_beam;
                }
                beam.insert(Point::new(x, y));
                width += 1;
                if !failed_test && width == 100 {
                    let test = Point::new(x, y - 99);
                    if beam.contains(&test) {
                        let closest_x = x - 99;
                        let closest_y = y - 99;
                        let answer = (10000 * closest_x) + closest_y;
                        println!(
                            "part 2: {:?} - answer is {}",
                            Point::new(closest_x, closest_y),
                            answer
                        );
                        return;
                    } else {
                        failed_test = true;
                    }
                }
            } else if prev_in_beam {
                break;
            }
        }
    }
}
