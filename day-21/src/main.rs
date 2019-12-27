mod droid;
mod intcode;

use intcode::{load_program, Computer};

use std::fs::File;
use std::io::Read;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem = load_program("./springdroid.intcode");
    let mut cpu = Computer::new(&mem);
    cpu.run();
    cpu.flush_output();
    load_springscript(&mut cpu, "./part1.springscript");
    cpu.run();
    let result = cpu.flush_output();
    println!("part 1: {:?}", result);
}

fn part2() {
    let mem = load_program("./springdroid.intcode");
    let mut cpu = Computer::new(&mem);
    cpu.run();
    cpu.flush_output();
    load_springscript(&mut cpu, "./part2.springscript");
    cpu.run();
    let result = cpu.flush_output();
    println!("part 2: {:?}", result);
}

fn load_springscript(cpu: &mut Computer, filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    cpu.send_message(&contents);
}
