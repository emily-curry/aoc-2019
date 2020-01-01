pub enum OperationResult {
  Continue { advance: bool },
  Yield,
  Halt,
}

impl Default for OperationResult {
  fn default() -> Self {
    OperationResult::Continue { advance: true }
  }
}
