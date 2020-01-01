use super::intcode_result::{IntCodeResult, IntCodeResultKind};
use super::operation::{Operation, OperationMode};
use super::operation_result::OperationResult;
use aoc_util::digits;

pub struct IntCode {
  data: Vec<isize>,
  index: usize,
  input: Vec<isize>,
  input_index: usize,
  output: Vec<isize>,
}

impl IntCode {
  pub fn from_vec(raw: Vec<isize>) -> IntCode {
    IntCode {
      data: raw,
      index: 0,
      input: Vec::new(),
      input_index: 0,
      output: Vec::new(),
    }
  }

  pub fn from_string(raw: &str) -> IntCode {
    let data: Vec<isize> = raw
      .split(",")
      .map(|x| x.parse::<isize>().unwrap())
      .collect();
    IntCode {
      data,
      index: 0,
      input: Vec::new(),
      input_index: 0,
      output: Vec::new(),
    }
  }

  pub fn input(&mut self, inputs: &Vec<isize>) {
    self.input.extend(inputs.iter());
  }

  pub fn execute(&mut self) -> Result<IntCodeResult, ()> {
    loop {
      let result = match self.operation() {
        Operation::Add(i) => self.exec_add(&i),
        Operation::Multiply(i) => self.exec_multiply(&i),
        Operation::Input => self.exec_input(),
        Operation::Output(i) => self.exec_output(&i),
        Operation::JumpIfTrue(i) => self.exec_jump_if_true(&i),
        Operation::JumpIfFalse(i) => self.exec_jump_if_false(&i),
        Operation::LessThan(i) => self.exec_less_than(&i),
        Operation::Equals(i) => self.exec_equals(&i),
        Operation::Halt => self.exec_halt(),
      }?;
      match result {
        OperationResult::Continue { advance } => {
          if advance == true {
            self.advance();
          }
        }
        OperationResult::Halt => {
          return Ok(IntCodeResult {
            kind: IntCodeResultKind::Halt,
            first: &self.data[0],
            output: &self.output,
          })
        }
        OperationResult::Yield => {
          return Ok(IntCodeResult {
            kind: IntCodeResultKind::Yield,
            first: &self.data[0],
            output: &self.output,
          })
        }
      }
    }
  }

  fn operation(&self) -> Operation {
    let opcode = digits(self.data[self.index]);
    let op = opcode[0] + (opcode.get(1).cloned().unwrap_or(0) * 10);
    let mut params = [0; 2]; // Increase array len if more params needed
    for i in 0..params.len() {
      params[i] = opcode.get(i + 2).cloned().unwrap_or(0);
    }

    let el0 = OperationMode::from(params[0]);
    let el1 = OperationMode::from(params[1]);
    match op {
      1 => Operation::Add([el0, el1]),
      2 => Operation::Multiply([el0, el1]),
      3 => Operation::Input,
      4 => Operation::Output([el0]),
      5 => Operation::JumpIfTrue([el0, el1]),
      6 => Operation::JumpIfFalse([el0, el1]),
      7 => Operation::LessThan([el0, el1]),
      8 => Operation::Equals([el0, el1]),
      99 => Operation::Halt,
      _ => panic!("No operation for code: {}", op),
    }
  }

  fn operation_length(&self) -> usize {
    match self.operation() {
      Operation::Add(_)
      | Operation::Multiply(_)
      | Operation::LessThan(_)
      | Operation::Equals(_) => 4,
      Operation::JumpIfTrue(_) | Operation::JumpIfFalse(_) => 3,
      Operation::Input | Operation::Output(_) => 2,
      Operation::Halt => 1,
    }
  }

  fn advance(&mut self) {
    self.index += self.operation_length();
  }

  fn is_safe(&self, index: isize) -> bool {
    index >= 0 && index as usize <= self.data.len() - 1
  }

  fn read(&self, index: usize, mode: &OperationMode) -> Result<isize, ()> {
    match mode {
      OperationMode::Position => {
        if !self.is_safe(index as isize) || !self.is_safe(self.data[index]) {
          return Err(());
        }
        Ok(self.data[self.data[index] as usize])
      }
      OperationMode::Immediate => {
        if !self.is_safe(index as isize) {
          return Err(());
        }
        Ok(self.data[index])
      }
    }
  }

  fn write(&mut self, index: usize, value: isize) -> Result<(), ()> {
    if !self.is_safe(index as isize) || !self.is_safe(self.data[index]) {
      return Err(());
    }
    let out = self.data[index] as usize;
    self.data[out] = value;
    Ok(())
  }

  fn read_input(&mut self) -> Result<isize, ()> {
    if self.input_index > self.input.len() - 1 {
      return Err(());
    }
    let result = Ok(self.input[self.input_index]);
    self.input_index += 1;
    result
  }

  fn exec_add(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let val = self.read(self.index + 1, &modes[0])? + self.read(self.index + 2, &modes[1])?;
    self.write(self.index + 3, val)?;
    Ok(Default::default())
  }

  fn exec_multiply(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let val = self.read(self.index + 1, &modes[0])? * self.read(self.index + 2, &modes[1])?;
    self.write(self.index + 3, val)?;
    Ok(Default::default())
  }

  fn exec_input(&mut self) -> Result<OperationResult, ()> {
    let input = self.read_input();
    match input {
      Ok(i) => {
        self.write(self.index + 1, i)?;
        Ok(Default::default())
      }
      Err(_) => Ok(OperationResult::Yield),
    }
  }

  fn exec_output(&mut self, modes: &[OperationMode; 1]) -> Result<OperationResult, ()> {
    let result = self.read(self.index + 1, &modes[0])?;
    self.output.push(result);
    Ok(Default::default())
  }

  fn exec_jump_if_true(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let val = self.read(self.index + 1, &modes[0])?;
    if val != 0 {
      let next_index = self.read(self.index + 2, &modes[1])?;
      self.index = next_index as usize;
      return Ok(OperationResult::Continue { advance: false });
    }
    Ok(Default::default())
  }

  fn exec_jump_if_false(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let val = self.read(self.index + 1, &modes[0])?;
    if val == 0 {
      let next_index = self.read(self.index + 2, &modes[1])?;
      self.index = next_index as usize;
      return Ok(OperationResult::Continue { advance: false });
    }
    Ok(Default::default())
  }

  fn exec_less_than(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let left = self.read(self.index + 1, &modes[0])?;
    let right = self.read(self.index + 2, &modes[1])?;
    let val = match left < right {
      true => 1,
      false => 0,
    };
    self.write(self.index + 3, val)?;
    Ok(Default::default())
  }

  fn exec_equals(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
    let left = self.read(self.index + 1, &modes[0])?;
    let right = self.read(self.index + 2, &modes[1])?;
    let val = match left == right {
      true => 1,
      false => 0,
    };
    self.write(self.index + 3, val)?;
    Ok(Default::default())
  }

  fn exec_halt(&self) -> Result<OperationResult, ()> {
    Ok(OperationResult::Halt)
  }
}

impl Clone for IntCode {
  fn clone(&self) -> IntCode {
    IntCode {
      data: self.data.clone(),
      index: self.index,
      input: self.input.clone(),
      input_index: self.input_index,
      output: self.output.clone(),
    }
  }
}
