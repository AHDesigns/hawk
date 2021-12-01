use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};
use tree_sitter::{Language, Parser};

extern "C" {
  fn tree_sitter_javascript() -> Language;
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer {
  name: String,
  lines: Vec<String>,
}

pub fn open_buffer(path: &Path) -> Result<Buffer, String> {
  highlight();
  match read_to_string(path) {
    Err(_) => Err(String::from(format!("could not read path: {:?}", path))),
    Ok(buf) => Ok(Buffer {
      // the hell is this??
      name: path.file_name().unwrap().to_owned().into_string().unwrap(),
      lines: buf.lines().map(|l| l.to_string()).collect(),
    }),
  }
}

fn highlight() {
  let mut parser = Parser::new();
  let language = unsafe { tree_sitter_javascript() };
  parser.set_language(language).unwrap();
  let source_code = "let x = 4";
  let tree = parser.parse(source_code, None).unwrap();
  let root_node = tree.root_node();
  println!("{}", &root_node.to_sexp())
}
