struct Digits {
  n: Option<i32>,
}

impl Digits {
  fn new(n: i32) -> Self {
    Digits { n: Some(n) }
  }
}

impl Iterator for Digits {
  type Item = i32;

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

fn to_digits(n: i32) -> Digits {
  Digits::new(n)
}

pub type Memory = Vec<i32>;

pub struct Computer {
  memory: Memory,
  pc: usize,
  halted: bool,
  input: Option<i32>,
  output: Option<i32>,
}

impl Computer {
  pub fn new(memory: Memory) -> Computer {
    Computer {
      memory,
      pc: 0,
      halted: false,
      input: None,
      output: None,
    }
  }

  pub fn get_memory(&self) -> Memory {
    self.memory.clone()
  }

  pub fn set_input(&mut self, input: i32) {
    self.input = Some(input);
  }

  pub fn get_output(&self) -> Option<i32> {
    self.output
  }

  pub fn run(&mut self) {
    while !self.halted {
      self.step();
    }
  }

  fn step(&mut self) {
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
      99 => self.exec_halt(),
      bad_op => panic!("Unknown op code {:?}", bad_op),
    };

    if self.pc == prev_pc {
      self.pc += inc;
    }
  }

  fn read(&self, addr: i32) -> i32 {
    self.memory[addr as usize]
  }

  fn write(&mut self, addr: i32, data: i32) {
    self.memory[addr as usize] = data;
  }

  fn read_pc(&self) -> i32 {
    self.memory[self.pc]
  }

  fn read_pc_offset(&self, offset: usize) -> i32 {
    self.memory[self.pc + offset]
  }

  fn get_op_code(&self) -> i32 {
    let mut op_code = self.read_pc();
    if op_code > 99 {
      let digits = to_digits(op_code).take(2).collect::<Vec<_>>();
      op_code = digits[0] + digits[1] * 10;
    }
    op_code
  }

  fn get_mode(&self, param_idx: usize) -> Option<i32> {
    assert!(param_idx >= 1 && param_idx <= 3);
    let digit_idx = param_idx + 1;
    to_digits(self.read_pc()).nth(digit_idx).or(Some(0))
  }

  fn read_param(&self, param_idx: usize) -> Option<i32> {
    self.get_mode(param_idx).and_then(|mode| {
      let param = self.read_pc_offset(param_idx);
      match mode {
        0 => Some(self.read(param)),
        1 => Some(param),
        bad_mode => panic!("Unexpected mode {}", bad_mode),
      }
    })
  }

  fn exec_add(&mut self) -> usize {
    self.execute(|x, y| x + y)
  }

  fn exec_mult(&mut self) -> usize {
    self.execute(|x, y| x * y)
  }

  fn exec_equals(&mut self) -> usize {
    self.execute(|x, y| if x == y { 1 } else { 0 })
  }

  fn exec_less_than(&mut self) -> usize {
    self.execute(|x, y| if x < y { 1 } else { 0 })
  }

  fn execute<T>(&mut self, operation: T) -> usize
  where
    T: Fn(i32, i32) -> i32,
  {
    let param1 = self.read_param(1);
    let param2 = self.read_param(2);

    let result = match (param1, param2) {
      (Some(in1), Some(in2)) => operation(in1, in2),
      _ => panic!("Failed to read all parameters"),
    };

    let addr = self.read_pc_offset(3);
    self.write(addr, result);
    4
  }

  fn exec_input(&mut self) -> usize {
    let addr = self.read_pc_offset(1);
    match self.input {
      Some(data) => self.write(addr, data),
      None => panic!("Tried to read input but it was None"),
    };
    2
  }

  fn exec_output(&mut self) -> usize {
    self.read_param(1).map(|data| {
      self.output = Some(data);
    });
    2
  }

  fn exec_jump_if_true(&mut self) -> usize {
    self.exec_jump(|data| data != 0)
  }

  fn exec_jump_if_false(&mut self) -> usize {
    self.exec_jump(|data| data == 0)
  }

  fn exec_jump<T>(&mut self, predicate: T) -> usize
  where
    T: Fn(i32) -> bool,
  {
    self.read_param(1).map(|data| {
      if predicate(data) {
        self.read_param(2).map(|addr| self.pc = addr as usize);
      }
    });
    3
  }

  fn exec_halt(&mut self) -> usize {
    self.halted = true;
    1
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn expect_program(initial_mem: Memory, expected_mem: Memory) {
    let mut cpu = Computer::new(initial_mem);
    cpu.run();
    assert_eq!(cpu.get_memory(), expected_mem);
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
  fn test_digits() {
    assert_eq!(to_digits(0).collect::<Vec<_>>(), vec![0]);
    assert_eq!(to_digits(1).collect::<Vec<_>>(), vec![1]);
    assert_eq!(to_digits(10).collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(to_digits(42).collect::<Vec<_>>(), vec![2, 4]);
    assert_eq!(to_digits(100).collect::<Vec<_>>(), vec![0, 0, 1]);
  }

  #[test]
  fn test_input_output() {
    let initial_mem = vec![3, 0, 4, 0, 99];
    let mut cpu = Computer::new(initial_mem);
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
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(8);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));

    let initial_mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));
  }

  #[test]
  fn test_lt_8() {
    let initial_mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(8);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));

    let initial_mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));
  }

  #[test]
  fn test_eq_8_immediate() {
    let initial_mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(8);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));

    let initial_mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));
  }

  #[test]
  fn test_lt_8_immediate() {
    let initial_mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(8);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));

    let initial_mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));
  }

  #[test]
  fn test_jump_position() {
    let initial_mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(0);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));

    let initial_mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));
  }

  #[test]
  fn test_jump_immediate() {
    let initial_mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(0);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(0));

    let initial_mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1));
  }

  #[test]
  fn test_compare_8() {
    let initial_mem = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(7);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(999));

    let initial_mem = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(8);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1000));

    let initial_mem = vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];
    let mut cpu = Computer::new(initial_mem);
    cpu.set_input(9);
    cpu.run();
    assert_eq!(cpu.get_output(), Some(1001));
  }
}
