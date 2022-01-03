use log::warn;

use crate::buffers::Buffer;
use crate::util::Pos;

struct Window {
  id: u8,
  buffer_ref: usize,
  width: u16,
  height: u16,
  // scroll_x: u8,
  // scroll_y: u8,
}

impl Window {
  pub fn new(id: u8, buffer_ref: usize, width: u16, height: u16) -> Self {
    Window {
      id,
      buffer_ref,
      width,
      height,
    }
  }

    pub fn draw() {
	
    }

  pub fn resize(&mut self, width: u16, height: u16) {
    self.width = width;
    self.height = height;
  }

  pub fn get_char_positions(&self, c: char, buffers: &Vec<Buffer>) -> Vec<Pos> {
    buffers
      .iter()
      .find(|b| b.id() == self.buffer_ref)
      .expect("could not find buffer")
      .find_char_positions(c)
  }
}

pub struct Display {
  windows: Vec<Window>,
  last_window_id: u8,
  width: u16,
  height: u16,
}

impl Display {
    pub fn draw(&self) {
	
    }
  pub fn new(width: u16, height: u16, buffer_ref: usize) -> Self {
    let mut display = Display {
      width,
      height,
      last_window_id: 0,
      windows: Vec::new(),
    };

    let first_window = display.create_window(buffer_ref);

    display.windows.push(first_window);

    display
  }

  pub fn resize(&mut self, width: u16, height: u16) {
    self.width = width;
    self.height = height;

    match self.windows.len() {
      1 => self.windows.get_mut(0).unwrap().resize(width, height),
      2 => {
        self
          .windows
          .iter_mut()
          .for_each(|w| w.resize(width / 2, height));
      }
      _ => panic!("more than 2 windows isn't supported"),
    }
  }

  pub fn split_window(&mut self, buffer_ref: usize) {
    let w = self.create_window(buffer_ref);
    self.windows.push(w);
  }

  fn create_window(&mut self, buffer_ref: usize) -> Window {
    self.last_window_id += 1;

    match self.windows.len() {
      0 => Window::new(self.last_window_id, buffer_ref, self.width, self.height),
      1 => {
        let win_1 = self.windows.get_mut(0).unwrap();
        win_1.resize(self.width / 2, self.height);

        Window::new(self.last_window_id, buffer_ref, self.width / 2, self.height)
      }
      _ => panic!("can't create more than 2 windows"),
    }
  }

  fn get_window(&self, id: u8) -> Option<&Window> {
    self.windows.iter().find(|w| w.id == id)
  }

  fn get_char_positions(&self, ch: char, window_id: Option<u8>, buffers: &Vec<Buffer>) -> Vec<Pos> {
    match window_id {
      Some(id) => match self.get_window(id) {
        Some(window) => window.get_char_positions(ch, buffers),
        None => Vec::new(),
      },
      None => self
        .windows
        .iter()
        .flat_map(|w| w.get_char_positions(ch, buffers))
        .collect(),
    }
  }
}
