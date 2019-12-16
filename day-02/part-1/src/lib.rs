pub type Memory = Vec<usize>;

pub struct Computer {
  memory: Memory,
  pc: usize,
  halted: bool,
}

impl Computer {
  pub fn new(memory: Memory) -> Computer {
    Computer {
      memory,
      pc: 0,
      halted: false,
    }
  }

  pub fn memory(&self) -> Memory {
    self.memory.clone()
  }

  pub fn output(&self) -> usize {
    self.memory[0]
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

    let op = self.memory[self.pc];
    match op {
      1 => self.add(),
      2 => self.mult(),
      99 => self.halt(),
      op_code => panic!("Unknown op code {:?}", op_code),
    }

    self.pc += 4;
  }

  fn add(&mut self) {
    self.execute(
      self.memory[self.pc + 1],
      self.memory[self.pc + 2],
      self.memory[self.pc + 3],
      |x, y| x + y,
    );
  }

  fn mult(&mut self) {
    self.execute(
      self.memory[self.pc + 1],
      self.memory[self.pc + 2],
      self.memory[self.pc + 3],
      |x, y| x * y,
    );
  }

  fn execute<T>(&mut self, in_addr1: usize, in_addr2: usize, out_addr: usize, operation: T)
  where
    T: Fn(usize, usize) -> usize,
  {
    let in1 = self.memory[in_addr1];
    let in2 = self.memory[in_addr2];

    let result = operation(in1, in2);

    self.memory[out_addr] = result;
  }

  fn halt(&mut self) {
    self.halted = true
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn expect_program(initial_mem: Memory, expected_mem: Memory) {
    let mut cpu = Computer::new(initial_mem);
    cpu.run();
    assert_eq!(cpu.memory(), expected_mem);
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
}
