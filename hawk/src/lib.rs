use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Parser};

extern "C" {
  fn tree_sitter_javascript() -> Language;
}

pub mod buffers;
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
/// This struct holds the state of the app.
pub struct App {
  pub buffers: Vec<Buffer>,
}

impl App {
  pub fn new() -> Self {
    App {
      buffers: Vec::new(),
    }
  }

  pub fn create_buffer(&mut self, name: String) {
    self.buffers.push(Buffer::new(name));
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

mod display {
  use crate::util::Pos;
  use std::rc::Rc;

  use crate::buffers::Buffer;

  struct Window {
    id: u8,
    buffer_ref: Rc<Buffer>,
    // width: u8,
    // height: u8,
    // scroll_x: u8,
    // scroll_y: u8,
  }

  impl Window {
    fn new(id: u8, buffer: Rc<Buffer>) -> Self {
      Window {
        id,
        buffer_ref: buffer,
      }
    }

    fn get_char_positions(&self, c: char) -> Vec<Pos> {
      self.buffer_ref.find_char_positions(c)
    }
  }

  struct Display {
    last_window_id: u8,
    pub windows: Vec<Window>,
  }

  impl Display {
    pub fn new(buffer_ref: Rc<Buffer>) -> Self {
      let mut display = Display {
        last_window_id: 0,
        windows: Vec::new(),
      };

      let first_window = display.create_window(buffer_ref);

      display.windows.push(first_window);

      display
    }

    fn create_window(&mut self, buffer_ref: Rc<Buffer>) -> Window {
      self.last_window_id += 1;

      Window::new(self.last_window_id, buffer_ref)
    }

    fn get_window(&self, id: u8) -> Option<&Window> {
      self.windows.iter().find(|w| w.id == id)
    }

    fn get_char_positions(&self, ch: char, window_id: Option<u8>) -> Vec<Pos> {
      match window_id {
        Some(id) => match self.get_window(id) {
          Some(window) => window.get_char_positions(ch),
          None => Vec::new(),
        },
        None => self
          .windows
          .iter()
          .flat_map(|w| w.get_char_positions(ch))
          .collect(),
      }
    }
  }
}
