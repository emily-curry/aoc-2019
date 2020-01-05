pub trait CharEnum
where
  Self: Sized,
{
  fn from_char(input: &char) -> Result<Self, ()>;

  fn to_char(&self) -> char;
}
