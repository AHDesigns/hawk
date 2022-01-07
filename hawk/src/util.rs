#[derive(Debug, Eq, PartialEq)]
pub struct Pos {
  pub row: u16,
  pub column: u16,
}

impl Pos {
  pub fn new(row: u16, column: u16) -> Self {
    Pos { row, column }
  }
}
