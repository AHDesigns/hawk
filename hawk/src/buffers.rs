pub mod contents {
  use tree_sitter::{Language, Parser};

  use crate::util::Pos;

  use super::BufferId;

  extern "C" {
    fn tree_sitter_javascript() -> Language;
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

  pub struct Content {
    text: Vec<String>,
  }

  impl Content {
    pub fn new(text: String) -> Self {
      Content {
        text: text.lines().map(|l| l.to_string()).collect(),
      }
    }

    pub fn text(&self) -> String {
      self.text.join("\n")
    }

    pub fn default() -> Self {
      Content {
        text: vec!["".to_string()],
      }
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

    pub fn remove_text(&mut self, line: u16) {
      if !self.text.is_empty() {
        if let Some(ln) = self.text.get_mut(BufferId::from(line)) {
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
              row: row as u16,
              column: column as u16,
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
  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_highlight() {
      assert_eq!(
        "(program (lexical_declaration (variable_declarator name: (identifier) value: (number))))",
        highlight()
      );
    }

    #[test]
    fn test_find_char_positions() {
      let mut content = Content::default();
      content.append_text("here is some text".to_string());
      println!("{}", &content.text());

      assert_eq!(
        content.find_char_positions('s'),
        vec![Pos::new(0, 6), Pos::new(0, 8)]
      );

      content.line_break();
      content.append_text("sammy snake sss".to_string());

      assert_eq!(
        content.find_char_positions('s'),
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
}

use self::contents::Content;

pub type BufferId = usize;

pub struct Buffer {
  id: BufferId,
  name: String,
  pub content: contents::Content,
}

impl Buffer {
  pub fn id(&self) -> BufferId {
    self.id
  }

  pub fn new(id: BufferId, name: String, content: Content) -> Self {
    Buffer { name, id, content }
  }

  pub fn default(id: BufferId) -> Self {
    Buffer::new(id, format!("buffer-{}", &id), Content::default())
  }
}

#[cfg(test)]
mod tests {
  use super::Buffer;

  #[test]
  fn test_default_buffer() {
    let buff = Buffer::default(0);
    assert_eq!(buff.name, "buffer-0".to_string())
  }
}

// use std::{fs::read_to_string, path::Path};
// pub fn open_buffer(path: &Path) -> Result<Buffer, String> {
//   match read_to_string(path) {
//     Err(_) => Err(format!("could not read path: {:?}", path)),
//     Ok(buf) => Ok(Buffer::new(
//       // the hell is this??
//       path.file_name().unwrap().to_owned().into_string().unwrap(),
//       Some(buf.lines().map(|l| l.to_string()).collect()),
//     )),
//   }
// }
