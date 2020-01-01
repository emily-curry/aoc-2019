pub enum Operation {
  Add([OperationMode; 2]),
  Multiply([OperationMode; 2]),
  Input,
  Output([OperationMode; 1]),
  JumpIfTrue([OperationMode; 2]),
  JumpIfFalse([OperationMode; 2]),
  LessThan([OperationMode; 2]),
  Equals([OperationMode; 2]),
  Halt,
}

pub enum OperationMode {
  Position,
  Immediate,
}

impl OperationMode {
  pub fn from(i: isize) -> OperationMode {
    match i {
      0 => OperationMode::Position,
      1 => OperationMode::Immediate,
      _ => panic!("No operation mode: {}", i),
    }
  }
}
