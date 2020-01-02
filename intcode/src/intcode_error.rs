use super::operation::OperationMode;

#[derive(Debug, Clone)]
pub struct IntCodeError {
  pub kind: IntCodeErrorKind,
  pub opcode: isize,
  pub index: usize,
}

#[derive(Debug, Clone)]
pub enum IntCodeErrorKind {
  ReadOutOfRange { index: isize, mode: OperationMode },
  WriteOutOfRange { index: isize, mode: OperationMode },
  WriteInvalidOperationMode { mode: OperationMode },
}
