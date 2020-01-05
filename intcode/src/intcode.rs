use super::intcode_error::{IntCodeError, IntCodeErrorKind};
use super::intcode_result::{IntCodeResult, IntCodeResultKind};
use super::operation::{Operation, OperationMode};
use super::operation_result::OperationResult;
use aoc_util::ToDigits;

#[derive(Debug, Clone)]
pub struct IntCode {
  data: Vec<isize>,
  index: usize,
  input: Vec<isize>,
  input_index: usize,
  output: Vec<isize>,
  relative_base: isize,
}

impl IntCode {
  pub fn from_vec(raw: Vec<isize>) -> IntCode {
    IntCode {
      data: raw,
      index: 0,
      input: Vec::new(),
      input_index: 0,
      output: Vec::new(),
      relative_base: 0,
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
      relative_base: 0,
    }
  }

  pub fn input(&mut self, inputs: &Vec<isize>) {
    self.input.extend(inputs.iter());
  }

  pub fn execute(&mut self) -> Result<IntCodeResult, IntCodeError> {
    loop {
      let result = match self.operation() {
        Operation::Add(i) => self.exec_add(&i),
        Operation::Multiply(i) => self.exec_multiply(&i),
        Operation::Input(i) => self.exec_input(&i),
        Operation::Output(i) => self.exec_output(&i),
        Operation::JumpIfTrue(i) => self.exec_jump_if_true(&i),
        Operation::JumpIfFalse(i) => self.exec_jump_if_false(&i),
        Operation::LessThan(i) => self.exec_less_than(&i),
        Operation::Equals(i) => self.exec_equals(&i),
        Operation::RelBaseOffset(i) => self.exec_rel_base_offset(&i),
        Operation::Halt => self.exec_halt(),
      }?;
      match result {
        OperationResult::Continue { advance } => {
          if advance == true {
            self.advance();
          }
        }
        OperationResult::Halt => {
          return Ok(self.get_result(IntCodeResultKind::Halt));
        }
        OperationResult::Yield => {
          return Ok(self.get_result(IntCodeResultKind::Yield));
        }
      }
    }
  }

  fn operation(&self) -> Operation {
    let opcode = self.data[self.index].digits();
    let op = opcode[0] + (opcode.get(1).cloned().unwrap_or(0) * 10);
    let mut params = [0; 3]; // Increase array len if more params needed
    for i in 0..params.len() {
      params[i] = opcode.get(i + 2).cloned().unwrap_or(0);
    }

    let el0 = OperationMode::from(params[0]);
    let el1 = OperationMode::from(params[1]);
    let el2 = OperationMode::from(params[2]);
    match op {
      1 => Operation::Add([el0, el1, el2]),
      2 => Operation::Multiply([el0, el1, el2]),
      3 => Operation::Input([el0]),
      4 => Operation::Output([el0]),
      5 => Operation::JumpIfTrue([el0, el1]),
      6 => Operation::JumpIfFalse([el0, el1]),
      7 => Operation::LessThan([el0, el1, el2]),
      8 => Operation::Equals([el0, el1, el2]),
      9 => Operation::RelBaseOffset([el0]),
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
      Operation::Input(_) | Operation::Output(_) | Operation::RelBaseOffset(_) => 2,
      Operation::Halt => 1,
    }
  }

  fn advance(&mut self) {
    self.index += self.operation_length();
  }

  fn read(&self, index: usize, mode: &OperationMode) -> Result<isize, IntCodeError> {
    match mode {
      OperationMode::Position => {
        let pos = *self.data.get(index).unwrap_or(&0);
        if pos < 0 {
          return Err(self.get_error(IntCodeErrorKind::ReadOutOfRange {
            index: pos,
            mode: mode.clone(),
          }));
        }
        Ok(*self.data.get(pos as usize).unwrap_or(&0))
      }
      OperationMode::Immediate => Ok(*self.data.get(index).unwrap_or(&0)),
      OperationMode::Relative => {
        let pos = *self.data.get(index).unwrap_or(&0);
        let rel = pos + self.relative_base;
        if rel < 0 {
          return Err(self.get_error(IntCodeErrorKind::ReadOutOfRange {
            index: rel,
            mode: mode.clone(),
          }));
        }
        Ok(*self.data.get(rel as usize).unwrap_or(&0))
      }
    }
  }

  fn write(
    &mut self,
    index: usize,
    mode: &OperationMode,
    value: isize,
  ) -> Result<(), IntCodeError> {
    let out = match mode {
      OperationMode::Position => {
        let out = *self.data.get(index).unwrap_or(&0);
        if out < 0 {
          return Err(self.get_error(IntCodeErrorKind::WriteOutOfRange {
            index: out,
            mode: mode.clone(),
          }));
        }
        Ok(out as usize)
      }
      OperationMode::Immediate => {
        Err(self.get_error(IntCodeErrorKind::WriteInvalidOperationMode { mode: mode.clone() }))
      }
      OperationMode::Relative => {
        let pos = *self.data.get(index).unwrap_or(&0);
        let out = pos + self.relative_base;
        if out < 0 {
          return Err(self.get_error(IntCodeErrorKind::WriteOutOfRange {
            index: out,
            mode: mode.clone(),
          }));
        }
        Ok(out as usize)
      }
    }?;
    if out > self.data.len() - 1 {
      self.data.resize(out as usize + 1, 0);
    }
    self.data[out as usize] = value;
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

  fn exec_add(&mut self, modes: &[OperationMode; 3]) -> Result<OperationResult, IntCodeError> {
    let val = self.read(self.index + 1, &modes[0])? + self.read(self.index + 2, &modes[1])?;
    self.write(self.index + 3, &modes[2], val)?;
    Ok(Default::default())
  }

  fn exec_multiply(&mut self, modes: &[OperationMode; 3]) -> Result<OperationResult, IntCodeError> {
    let val = self.read(self.index + 1, &modes[0])? * self.read(self.index + 2, &modes[1])?;
    self.write(self.index + 3, &modes[2], val)?;
    Ok(Default::default())
  }

  fn exec_input(&mut self, modes: &[OperationMode; 1]) -> Result<OperationResult, IntCodeError> {
    let input = self.read_input();
    match input {
      Ok(i) => {
        self.write(self.index + 1, &modes[0], i)?;
        Ok(Default::default())
      }
      Err(_) => Ok(OperationResult::Yield),
    }
  }

  fn exec_output(&mut self, modes: &[OperationMode; 1]) -> Result<OperationResult, IntCodeError> {
    let result = self.read(self.index + 1, &modes[0])?;
    self.output.push(result);
    Ok(Default::default())
  }

  fn exec_jump_if_true(
    &mut self,
    modes: &[OperationMode; 2],
  ) -> Result<OperationResult, IntCodeError> {
    let val = self.read(self.index + 1, &modes[0])?;
    if val != 0 {
      let next_index = self.read(self.index + 2, &modes[1])?;
      self.index = next_index as usize;
      return Ok(OperationResult::Continue { advance: false });
    }
    Ok(Default::default())
  }

  fn exec_jump_if_false(
    &mut self,
    modes: &[OperationMode; 2],
  ) -> Result<OperationResult, IntCodeError> {
    let val = self.read(self.index + 1, &modes[0])?;
    if val == 0 {
      let next_index = self.read(self.index + 2, &modes[1])?;
      self.index = next_index as usize;
      return Ok(OperationResult::Continue { advance: false });
    }
    Ok(Default::default())
  }

  fn exec_less_than(
    &mut self,
    modes: &[OperationMode; 3],
  ) -> Result<OperationResult, IntCodeError> {
    let left = self.read(self.index + 1, &modes[0])?;
    let right = self.read(self.index + 2, &modes[1])?;
    let val = match left < right {
      true => 1,
      false => 0,
    };
    self.write(self.index + 3, &modes[2], val)?;
    Ok(Default::default())
  }

  fn exec_equals(&mut self, modes: &[OperationMode; 3]) -> Result<OperationResult, IntCodeError> {
    let left = self.read(self.index + 1, &modes[0])?;
    let right = self.read(self.index + 2, &modes[1])?;
    let val = match left == right {
      true => 1,
      false => 0,
    };
    self.write(self.index + 3, &modes[2], val)?;
    Ok(Default::default())
  }

  fn exec_rel_base_offset(
    &mut self,
    modes: &[OperationMode; 1],
  ) -> Result<OperationResult, IntCodeError> {
    let offset = self.read(self.index + 1, &modes[0])?;
    self.relative_base += offset;
    Ok(Default::default())
  }

  fn exec_halt(&self) -> Result<OperationResult, IntCodeError> {
    Ok(OperationResult::Halt)
  }

  fn get_result(&self, kind: IntCodeResultKind) -> IntCodeResult {
    IntCodeResult {
      kind,
      first: &self.data[0],
      output: &self.output,
    }
  }

  fn get_error(&self, kind: IntCodeErrorKind) -> IntCodeError {
    IntCodeError {
      kind,
      index: self.index,
      opcode: self.data[self.index],
    }
  }
}
