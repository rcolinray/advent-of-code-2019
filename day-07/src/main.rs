mod combos;
mod digits;
mod intcode;

use combos::combos;
use intcode::{Computer, Memory};

fn main() {
    let program = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 30, 55, 80, 101, 118, 199, 280, 361, 442, 99999, 3,
        9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 1002, 9, 4, 9, 101, 4, 9, 9, 1002, 9, 5, 9,
        1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 5, 9, 9, 1002, 9, 2, 9, 101, 3, 9, 9, 102, 4, 9, 9,
        1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 5, 9, 9, 102, 3, 9, 9, 101, 3, 9, 9, 4,
        9, 99, 3, 9, 1001, 9, 2, 9, 102, 4, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
    ];

    let max_signal = combos(vec![0, 1, 2, 3, 4])
        .iter()
        .map(|phases| try_phases_part1(&program, phases.clone()))
        .max();
    println!("part 1: {:?}", max_signal);

    let max_feedback_signal = combos(vec![5, 6, 7, 8, 9])
        .iter()
        .map(|phases| try_phases_part2(&program, phases.clone()))
        .max();
    println!("part 2: {:?}", max_feedback_signal);
}

fn try_phases_part1(program: &Memory, phases: Vec<i32>) -> i32 {
    let mut input = 0;
    for phase in phases {
        let mut cpu = Computer::new(program.clone());
        cpu.set_input(phase);
        cpu.step();
        cpu.set_input(input);
        cpu.run();
        input = cpu.get_output().unwrap();
    }
    input
}

fn try_phases_part2(program: &Memory, phases: Vec<i32>) -> i32 {
    let mut cpus = phases
        .iter()
        .map(|phase| {
            let mut cpu = Computer::new(program.clone());
            cpu.set_input(*phase);
            cpu.step();
            cpu
        })
        .collect::<Vec<_>>();

    let mut io_bus = phases
        .iter()
        .map(|_phase| None)
        .collect::<Vec<Option<i32>>>();
    io_bus[0] = Some(0);

    let mut output = 0;

    loop {
        if cpus.iter().all(|cpu| cpu.is_halted()) {
            break;
        }

        for (index, cpu) in cpus.iter_mut().enumerate() {
            let input = io_bus[index];
            io_bus[index] = None;

            match input {
                Some(data) => cpu.set_input(data),
                None => (),
            };

            cpu.step();

            io_bus[index] = cpu.get_output();
        }

        let mut new_io_bus = io_bus.clone();
        let last_index = io_bus.len() - 1;
        for index in 0..last_index {
            new_io_bus[index + 1] = io_bus[index];
        }
        match io_bus[last_index] {
            Some(data) => output = data,
            None => (),
        };
        new_io_bus[0] = io_bus[last_index];
        io_bus = new_io_bus;
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_phases1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let output = try_phases_part1(&program, vec![4, 3, 2, 1, 0]);
        assert_eq!(output, 43210);
    }

    #[test]
    fn test_try_phases2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let output = try_phases_part1(&program, vec![0, 1, 2, 3, 4]);
        assert_eq!(output, 54321);
    }

    #[test]
    fn test_try_phases3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let output = try_phases_part1(&program, vec![1, 0, 4, 3, 2]);
        assert_eq!(output, 65210);
    }

    #[test]
    fn test_try_phases4() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let output = try_phases_part2(&program, vec![9, 8, 7, 6, 5]);
        assert_eq!(output, 139629729);
    }

    #[test]
    fn test_try_phases5() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let output = try_phases_part2(&program, vec![9, 7, 8, 5, 6]);
        assert_eq!(output, 18216);
    }
}
