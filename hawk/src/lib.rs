use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Parser};

extern "C" {
  fn tree_sitter_javascript() -> Language;
}

pub mod buffers;

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
    assert_eq!(buff.lines, vec!["here is some text"]);
  }
}
