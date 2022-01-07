#[derive(Debug, Eq, PartialEq)]
pub struct Pos {
  pub row: u8,
  pub column: u8,
}

impl Pos {
  pub fn new(row: u8, column: u8) -> Self {
    Pos { row, column }
  }
}
