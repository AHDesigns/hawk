use std::{fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

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
  match read_to_string(path) {
    Err(_) => Err(String::from(format!("could not read path: {:?}", path))),
    Ok(buf) => Ok(Buffer {
      // the hell is this??
      name: path.file_name().unwrap().to_owned().into_string().unwrap(),
      lines: buf.lines().map(|l| l.to_string()).collect(),
    }),
  }
}
