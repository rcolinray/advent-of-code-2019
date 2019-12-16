use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::iter::repeat;

pub type Memory = Vec<i64>;

pub fn load_program(filename: &str) -> Memory {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split(",")
        .map(|text| text.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

pub struct Computer {
    memory: Memory,
    pc: usize,
    base: i64,
    halted: bool,
    blocked: bool,
    input: VecDeque<i64>,
    // input: Option<i64>,
    output: VecDeque<i64>,
}

impl Computer {
    pub fn new(memory: &Memory) -> Computer {
        let mut new_memory = memory.clone();
        new_memory.extend(repeat(0).take(10000));
        Computer {
            memory: new_memory,
            pc: 0,
            base: 0,
            halted: false,
            blocked: false,
            input: VecDeque::new(),
            // input: None,
            output: VecDeque::new(),
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    #[allow(dead_code)]
    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn set_input(&mut self, input: i64) {
        self.input.push_back(input);
        self.blocked = false;
        // self.input = Some(input);
    }

    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    #[allow(dead_code)]
    pub fn run(&mut self) {
        while !self.halted && !self.blocked {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let prev_pc = self.pc;

        let op_code = self.get_op_code();

        let inc = match op_code {
            1 => self.exec_add(),
            2 => self.exec_mult(),
            3 => self.exec_input(),
            4 => self.exec_output(),
            5 => self.exec_jump_if_true(),
            6 => self.exec_jump_if_false(),
            7 => self.exec_less_than(),
            8 => self.exec_equals(),
            9 => self.exec_set_base(),
            99 => self.exec_halt(),
            bad_op => panic!("Unknown op code {:?}", bad_op),
        };

        if self.pc == prev_pc {
            self.pc += inc;
        }
    }

    fn data_to_addr(&self, data: i64) -> usize {
        if data < 0 {
            0
        } else {
            data as usize
        }
    }

    fn read(&self, addr: i64) -> i64 {
        let final_addr = self.data_to_addr(addr);
        self.memory[final_addr]
    }

    fn read_relative(&self, addr: i64) -> i64 {
        let final_addr = self.data_to_addr(self.base + addr);
        self.memory[final_addr]
    }

    fn write(&mut self, addr: i64, data: i64) {
        let final_addr = self.data_to_addr(addr);
        self.memory[final_addr] = data;
    }

    fn write_relative(&mut self, addr: i64, data: i64) {
        let final_addr = self.data_to_addr(self.base + addr);
        self.memory[final_addr] = data;
    }

    fn read_pc(&self) -> i64 {
        self.memory[self.pc]
    }

    fn read_pc_offset(&self, offset: usize) -> i64 {
        self.memory[self.pc + offset]
    }

    fn get_op_code(&self) -> i64 {
        let mut op_code = self.read_pc();
        if op_code > 99 {
            let digits = to_digits(op_code).take(2).collect::<Vec<_>>();
            op_code = digits[0] + digits[1] * 10;
        }
        op_code
    }

    fn get_mode(&self, param_idx: usize) -> Option<i64> {
        assert!(param_idx >= 1 && param_idx <= 3);
        let digit_idx = param_idx + 1;
        to_digits(self.read_pc()).nth(digit_idx)
    }

    fn read_param_in(&self, param_idx: usize) -> Option<i64> {
        self.get_mode(param_idx).or(Some(0)).and_then(|mode| {
            let param = self.read_pc_offset(param_idx);
            match mode {
                0 => Some(self.read(param)),
                1 => Some(param),
                2 => Some(self.read_relative(param)),
                bad_mode => panic!("Unexpected mode {}", bad_mode),
            }
        })
    }

    fn write_param_out(&mut self, param_idx: usize, data: i64) {
        let mode = self.get_mode(param_idx).or(Some(1));
        let param = self.read_pc_offset(param_idx);
        match mode {
            Some(1) => self.write(param, data),
            Some(2) => self.write_relative(param, data),
            bad_mode => panic!("Unexpected mode {:?}", bad_mode),
        }
    }

    fn exec_add(&mut self) -> usize {
        // println!(
        //     "add {} {} {}",
        //     self.read_pc_offset(1),
        //     self.read_pc_offset(2),
        //     self.read_pc_offset(3)
        // );
        self.execute(|x, y| x + y)
    }

    fn exec_mult(&mut self) -> usize {
        // println!(
        //     "mul {} {} {}",
        //     self.read_pc_offset(1),
        //     self.read_pc_offset(2),
        //     self.read_pc_offset(3)
        // );
        self.execute(|x, y| x * y)
    }

    fn exec_equals(&mut self) -> usize {
        // println!(
        //     "eq? {} {} {}",
        //     self.read_pc_offset(1),
        //     self.read_pc_offset(2),
        //     self.read_pc_offset(3)
        // );
        self.execute(|x, y| if x == y { 1 } else { 0 })
    }

    fn exec_less_than(&mut self) -> usize {
        // println!(
        //     "lt? {} {} {}",
        //     self.read_pc_offset(1),
        //     self.read_pc_offset(2),
        //     self.read_pc_offset(3)
        // );
        self.execute(|x, y| if x < y { 1 } else { 0 })
    }

    fn execute<T>(&mut self, operation: T) -> usize
    where
        T: Fn(i64, i64) -> i64,
    {
        let param1 = self.read_param_in(1);
        let param2 = self.read_param_in(2);

        let result = match (param1, param2) {
            (Some(in1), Some(in2)) => operation(in1, in2),
            _ => panic!("Failed to read all parameters"),
        };

        self.write_param_out(3, result);
        4
    }

    fn exec_input(&mut self) -> usize {
        // println!("inp {}", self.read_pc_offset(1));
        if let Some(data) = self.input.pop_front() {
            // if let Some(data) = self.input {
            self.blocked = false;
            self.write_param_out(1, data);
            2
        } else {
            self.blocked = true;
            0 // stall until input is available
        }
    }

    fn exec_output(&mut self) -> usize {
        // println!("out {}", self.read_pc_offset(1));
        self.read_param_in(1)
            .map(|data| self.output.push_back(data));
        2
    }

    fn exec_jump_if_true(&mut self) -> usize {
        // println!("jit {} {}", self.read_pc_offset(1), self.read_pc_offset(2));
        self.exec_jump(|data| data != 0)
    }

    fn exec_jump_if_false(&mut self) -> usize {
        // println!("jif {} {}", self.read_pc_offset(1), self.read_pc_offset(2));
        self.exec_jump(|data| data == 0)
    }

    fn exec_jump<T>(&mut self, predicate: T) -> usize
    where
        T: Fn(i64) -> bool,
    {
        self.read_param_in(1).map(|data| {
            if predicate(data) {
                self.read_param_in(2).map(|addr| self.pc = addr as usize);
            }
        });
        3
    }

    fn exec_set_base(&mut self) -> usize {
        // println!("bas {}", self.read_pc_offset(1));
        self.read_param_in(1).map(|off| self.base += off);
        2
    }

    fn exec_halt(&mut self) -> usize {
        // println!("hcf");
        self.halted = true;
        1
    }
}

pub struct Digits {
    n: Option<i64>,
}

impl Digits {
    fn new(n: i64) -> Self {
        Digits { n: Some(n) }
    }
}

impl Iterator for Digits {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            None => None,
            Some(n) => {
                if n < 10 {
                    let next = Some(n);
                    self.n = None;
                    next
                } else {
                    let next = n % 10;
                    self.n = Some(n / 10);
                    Some(next)
                }
            }
        }
    }
}

pub fn to_digits(n: i64) -> Digits {
    Digits::new(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(to_digits(0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(to_digits(1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(to_digits(10).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(to_digits(42).collect::<Vec<_>>(), vec![2, 4]);
        assert_eq!(to_digits(100).collect::<Vec<_>>(), vec![0, 0, 1]);
    }

    fn expect_program(initial_mem: Memory, expected_mem: Memory) {
        let mut cpu = Computer::new(&initial_mem);
        cpu.run();
        assert_eq!(
            &cpu.get_memory()[0..initial_mem.len()],
            expected_mem.as_slice()
        );
    }

    #[test]
    fn test_program_1() {
        expect_program(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_program_2() {
        expect_program(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_program_3() {
        expect_program(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_program_4() {
        expect_program(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_input_output() {
        let initial_mem = vec![3, 0, 4, 0, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(42);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(42));
    }

    #[test]
    fn test_modes() {
        expect_program(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_equal_8() {
        let initial_mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(8);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));

        let initial_mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));
    }

    #[test]
    fn test_lt_8() {
        let initial_mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(8);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));

        let initial_mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));
    }

    #[test]
    fn test_eq_8_immediate() {
        let initial_mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(8);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));

        let initial_mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));
    }

    #[test]
    fn test_lt_8_immediate() {
        let initial_mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(8);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));

        let initial_mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));
    }

    #[test]
    fn test_jump_position() {
        let initial_mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(0);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));

        let initial_mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));
    }

    #[test]
    fn test_jump_immediate() {
        let initial_mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(0);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(0));

        let initial_mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1));
    }

    #[test]
    fn test_compare_8() {
        let initial_mem = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(7);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(999));

        let initial_mem = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(8);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1000));

        let initial_mem = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut cpu = Computer::new(&initial_mem);
        cpu.set_input(9);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1001));
    }

    #[test]
    fn test_blocking_input() {
        let initial_mem = vec![3, 0, 99];
        let mut cpu = Computer::new(&initial_mem);
        cpu.step();
        cpu.step();
        cpu.step();
        assert_eq!(cpu.pc, 0);
        cpu.set_input(42);
        cpu.step();
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_relative1() {
        let mem = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut cpu = Computer::new(&mem);
        cpu.run();
        let mut copy = Vec::new();
        loop {
            match cpu.get_output() {
                Some(data) => copy.push(data),
                None => break,
            };
        }
        assert_eq!(copy, mem);
    }

    #[test]
    fn test_relative2() {
        let mem = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut cpu = Computer::new(&mem);
        cpu.run();
        let out_digits = match cpu.get_output() {
            Some(data) => to_digits(data).collect::<Vec<_>>(),
            None => vec![],
        };
        assert_eq!(out_digits.len(), 16);
    }

    #[test]
    fn test_relative3() {
        let mem = vec![104, 1125899906842624, 99];
        let mut cpu = Computer::new(&mem);
        cpu.run();
        assert_eq!(cpu.get_output(), Some(1125899906842624));
    }
}
