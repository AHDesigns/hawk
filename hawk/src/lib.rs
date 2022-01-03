use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Parser};

extern "C" {
  fn tree_sitter_javascript() -> Language;
}

pub mod buffers;
mod display;
pub mod logger;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PointInSpace {
  x: u32,
  y: u32,
}

pub fn window_fn(label: &str, x: u32, point_in_space: PointInSpace) {
  println!("x {}", x);
  println!("p {:?}", point_in_space);
  println!("Window {}", label)
}

fn highlight() -> String {
  let mut parser = Parser::new();
  let language = unsafe { tree_sitter_javascript() };
  parser.set_language(language).unwrap();
  let source_code = "let x = 4";
  let tree = parser.parse(source_code, None).unwrap();
  let root_node = tree.root_node();
  root_node.to_sexp()
}

#[cfg(test)]
mod tests {
  use crate::*;
  use std::path::Path;

  #[test]
  fn it_works() {
    assert_eq!(
      "(program (lexical_declaration (variable_declarator name: (identifier) value: (number))))",
      highlight()
    );
  }

  #[test]
  fn open_buffer_test() {
    let buff = buffers::open_buffer(&Path::new("./test/buffers/simple.txt")).unwrap();
    assert_eq!(buff.name, "simple.txt");
    assert_eq!(buff.text, vec!["here is some text"]);
  }
}

use buffers::Buffer;
use display::Display;

/// This struct holds the state of the app.
pub struct App {
  pub buffers: Vec<Buffer>,
  pub display: Display,
}

impl App {
  pub fn new(default_buffers: Vec<Buffer>, screen_size: (u16, u16)) -> Self {
    let initial_buffer = default_buffers
      .get(0)
      .expect("no initial_buffer passed")
      .id();

    App {
      buffers: default_buffers,
      display: Display::new(screen_size.0, screen_size.1, initial_buffer),
    }
  }

  pub fn default(screen_size: (u16, u16)) -> Self {
    App::new(vec![Buffer::new("scratch".to_string(), None)], screen_size)
  }

  pub fn create_buffer(&mut self, name: String) {
    self.buffers.push(Buffer::new(name, None));
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
    pub row: u8,
    pub column: u8,
  }

  impl Pos {
    pub fn new(row: u8, column: u8) -> Self {
      Pos { row, column }
    }
  }
}
