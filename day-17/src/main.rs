mod intcode;
mod vector;

use intcode::{load_program, Computer};
use vector::Vector;

use std::char;
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem = load_program("./input.intcode");
    let mut cpu = Computer::new(&mem);
    cpu.run();

    let map = print_video(&mut cpu);

    let alignment: i64 = map
        .iter()
        .filter(|&point| is_intersection(*point, &map))
        .map(|&point| alignment_param(point))
        .sum();
    println!("part 1: {}", alignment);
}

fn print_video(cpu: &mut Computer) -> HashSet<Vector<i64>> {
    let mut x = 0;
    let mut y = 0;

    let mut map = HashSet::<Vector<i64>>::new();
    let mut output = String::new();

    while let Some(data) = cpu.get_output() {
        let pixel =
            char::from_u32(data as u32).expect("Failed to get character for intcode output");
        output.push(pixel);

        match pixel {
            '#' => {
                map.insert(Vector::new(x, y));
            }
            _ => (),
        };

        if pixel == '\n' {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }

    println!("{}", output);

    map
}

fn is_intersection(point: Vector<i64>, map: &HashSet<Vector<i64>>) -> bool {
    let north = point + Vector::new(-1, 0);
    let south = point + Vector::new(1, 0);
    let east = point + Vector::new(0, 1);
    let west = point + Vector::new(0, -1);

    map.contains(&north) && map.contains(&south) && map.contains(&east) && map.contains(&west)
}

fn alignment_param(intersection: Vector<i64>) -> i64 {
    intersection.x * intersection.y
}

fn part2() {
    let mut mem = load_program("./input.intcode");
    mem[0] = 2;
    let mut cpu = Computer::new(&mem);

    cpu.run();
    flush_output(&mut cpu);
    send_input(&mut cpu, "A,B,A,C,B,A,C,B,A,C\n");

    cpu.run();
    flush_output(&mut cpu);
    send_input(&mut cpu, "L,6,L,4,R,12\n");

    cpu.run();
    flush_output(&mut cpu);
    send_input(&mut cpu, "L,6,R,12,R,12,L,8\n");

    cpu.run();
    flush_output(&mut cpu);
    send_input(&mut cpu, "L,6,L,10,L,10,L,6\n");

    cpu.run();
    flush_output(&mut cpu);
    send_input(&mut cpu, "n\n");

    cpu.run();

    let mut result = 0;
    loop {
        if let Some(data) = cpu.get_output() {
            let pixel = char::from_u32(data as u32).unwrap();
            match pixel {
                '.' => (),
                '#' => (),
                '^' => (),
                '>' => (),
                'v' => (),
                '<' => (),
                '\n' => (),
                _ => result = data,
            };
        } else {
            break;
        }
    }

    println!("part 2: {:?}", result);
}

fn flush_output(cpu: &mut Computer) {
    loop {
        if let Some(data) = cpu.get_output() {
            print!("{}", char::from_u32(data as u32).unwrap());
        } else {
            break;
        }
    }
    io::stdout().flush().unwrap();
}

fn send_input(cpu: &mut Computer, message: &str) {
    for c in message.chars() {
        let data = c as i64;
        cpu.set_input(data);
    }
}
