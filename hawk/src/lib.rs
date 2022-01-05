mod buffers;
mod display;
pub mod logger;

use buffers::{contents::Content, Buffer};
use display::Display;

struct IdGenerator {
  last: usize,
}

impl IdGenerator {
  pub fn default() -> Self {
    IdGenerator { last: 0 }
  }

  pub fn next_id(&mut self) -> usize {
    let last = self.last;
    self.last += 1;
    last
  }
}

/// God struct
pub struct App {
  buff_id: IdGenerator,
  pub buffers: Vec<Buffer>,
  display: Display,
}

impl App {
  pub fn new(mut default_buffers: Vec<Buffer>, screen_size: (u16, u16)) -> Self {
    let initial_buffer = default_buffers
      .get(0)
      .expect("no initial_buffer passed")
      .id();

    let mut buff_id = IdGenerator::default();

    App {
      buff_id,
      buffers: default_buffers,
      display: Display::new(screen_size.0, screen_size.1, initial_buffer),
    }
  }

  pub fn create_buffer(&mut self, name: String) {
    todo!();
    // self.buffers.push(Buffer::new(name, None));
  }
}

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Forward,
  Back,
}

#[derive(Debug)]
pub enum HawkEvent {
  Quit,
  Insert(char),
  Enter,
  Delete,
  Move(Direction),
  Ping,
  Slow,
  Resize((u16, u16)),
}

mod util {
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
}
