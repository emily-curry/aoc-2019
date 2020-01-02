#[derive(Debug, Clone)]
pub enum Operation {
  Add([OperationMode; 3]),
  Multiply([OperationMode; 3]),
  Input([OperationMode; 1]),
  Output([OperationMode; 1]),
  JumpIfTrue([OperationMode; 2]),
  JumpIfFalse([OperationMode; 2]),
  LessThan([OperationMode; 3]),
  Equals([OperationMode; 3]),
  RelBaseOffset([OperationMode; 1]),
  Halt,
}

#[derive(Debug, Clone)]
pub enum OperationMode {
  Position,
  Immediate,
  Relative,
}

impl OperationMode {
  pub fn from(i: isize) -> OperationMode {
    match i {
      0 => OperationMode::Position,
      1 => OperationMode::Immediate,
      2 => OperationMode::Relative,
      _ => panic!("No operation mode: {}", i),
    }
  }
}
