#[derive(Debug)]
pub enum IntCodeResultKind {
  Yield,
  Halt,
}

#[derive(Debug)]
pub struct IntCodeResult<'a> {
  pub kind: IntCodeResultKind,
  pub first: &'a isize,
  pub output: &'a Vec<isize>,
}
