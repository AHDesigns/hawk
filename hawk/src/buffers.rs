use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer {
  pub name: String,
  pub lines: Vec<String>,
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
