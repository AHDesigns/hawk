pub struct Buffer {
  pub name: String,
  pub text: Vec<String>,
}

impl Buffer {
  pub fn new(name: String) -> Self {
    Buffer {
      name,
      text: Vec::new(),
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
}
