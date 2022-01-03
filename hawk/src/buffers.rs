use crate::util::Pos;
use std::cell::Cell;
use std::{fs::read_to_string, path::Path};

mod modes {
  pub enum Mode {
    Simple,
    Special,
  }
}

thread_local!(static BUFFER_ID: Cell<usize> = Cell::new(0));

use modes::Mode;

pub struct Buffer {
  id: usize,
  pub name: String,
  pub text: Vec<String>,
  pub major_mode: Mode,
  pub minor_modes: Vec<Mode>,
}

impl Buffer {
  pub fn id(&self) -> usize {
    self.id
  }

  pub fn new(name: String, text: Option<Vec<String>>) -> Self {
    BUFFER_ID.with(|t_id| {
      let id = t_id.get();
      t_id.set(id + 1);

      Buffer {
        name,
        id,
        text: text.or_else(|| Some(Vec::new())).unwrap(),
        major_mode: Mode::Simple,
        minor_modes: Vec::new(),
      }
    })
  }

  pub fn append_text(&mut self, txt: String) {
    match self.text.is_empty() {
      true => self.text.push(txt),
      false => {
        let last_index = self.text.len();
        match self.text.get_mut(last_index - 1) {
          Some(last_line) => last_line.push_str(&txt),
          None => panic!("shouldn't happen"),
        }
      }
    }
  }

  pub fn remove_text(&mut self, line: u8) {
    if !self.text.is_empty() {
      if let Some(ln) = self.text.get_mut(line as usize) {
        ln.pop();
      }
    }
  }

  pub fn line_break(&mut self) {
    self.text.push("".to_string())
  }

  pub fn find_char_positions(&self, c: char) -> Vec<Pos> {
    // this would also work for string patterns

    self
      .text
      .iter()
      .enumerate()
      .filter_map(|(row, l)| {
        let matches: Vec<Pos> = l
          .match_indices(c)
          .map(|(column, _)| Pos {
            row: row as u8,
            column: column as u8,
          })
          .collect();

        if matches.is_empty() {
          None
        } else {
          Some(matches)
        }
      })
      .flatten()
      .collect()
  }
}

pub fn open_buffer(path: &Path) -> Result<Buffer, String> {
  match read_to_string(path) {
    Err(_) => Err(format!("could not read path: {:?}", path)),
    Ok(buf) => Ok(Buffer::new(
      // the hell is this??
      path.file_name().unwrap().to_owned().into_string().unwrap(),
      Some(buf.lines().map(|l| l.to_string()).collect()),
    )),
  }
}

#[cfg(test)]
mod tests {
  use crate::{buffers::Buffer, util::Pos};

  #[test]
  fn test_find_char_positions() {
    let mut buffer = Buffer::new(String::from("name"), None);
    buffer.append_text("here is some text".to_string());

    assert_eq!(
      buffer.find_char_positions('s'),
      vec![Pos::new(0, 6), Pos::new(0, 8)]
    );

    buffer.line_break();
    buffer.append_text("sammy snake sss".to_string());

    assert_eq!(
      buffer.find_char_positions('s'),
      vec![
        Pos::new(0, 6),
        Pos::new(0, 8),
        Pos::new(1, 0),
        Pos::new(1, 6),
        Pos::new(1, 12),
        Pos::new(1, 13),
        Pos::new(1, 14)
      ]
    );
  }
}
