mod buffers;
mod display;
pub mod events;
pub mod logger;

use display::Display;
use events::*;

mod editor {
  use crate::buffers::{contents::Content, Buffer, BufferId};

  struct IdGenerator {
    last: BufferId,
  }

  impl IdGenerator {
    pub fn default() -> Self {
      IdGenerator { last: 0 }
    }

    pub fn next_id(&mut self) -> BufferId {
      let last = self.last;
      self.last += 1;
      last
    }
  }

  /// This is what the user is interacting with, and does most of the work
  pub struct Editor {
    buff_id: IdGenerator,
    pub buffers: Vec<Buffer>,
  }

  impl Editor {
    pub fn new() -> Self {
      let mut buff_id = IdGenerator::default();

      let buffers = vec![Buffer::new(
        buff_id.next_id(),
        "*scratch*".to_string(),
        Content::default(),
      )];

      Self { buff_id, buffers }
    }

    pub fn create_buffer(&mut self) {
      self.buffers.push(Buffer::new(
        self.buff_id.next_id(),
        "oijwef".to_string(),
        Content::default(),
      ));
    }
  }
}

use editor::Editor;

/// Holds the state of the application, handling the editor loop and
/// deligating to the editor appropriately
pub struct App {
  pub event_handler: EventListener,
  pub editor: Editor,
  pub display: Display,
}

impl App {
  pub fn new(screen_size: (u16, u16)) -> Self {
    let editor = Editor::new();
    let initial_buffer = editor
      .buffers
      .get(0)
      .expect("no initial_buffer passed")
      .id();

    Self {
      editor,
      display: Display::new(screen_size.0, screen_size.1, initial_buffer),
      event_handler: EventListener::default(),
    }
  }

  /// creates a simple App, mainly for testing
  pub fn default() -> Self {
    App::new((16, 16))
  }

  pub fn handle_event(&mut self, e: HawkEvent) {
    let mut ctx = Context {
      editor: &mut self.editor,
    };

    if let HawkEvent::Key(k) = e {
      let keypress = &k.to_string();
      self.event_handler.keymap_handler.handle(&mut ctx, keypress);
    }
  }
}

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Forward,
  Back,
  None,
}

#[derive(Debug)]
pub enum HawkEvent {
  Quit,
  Key(char),
  Enter,
  Delete,
  Ping,
  Slow,
  Resize((u16, u16)),
  Up,
  Down,
  Forward,
  Back,
}

mod util {
  #[derive(Debug, Eq, PartialEq)]
  pub struct Pos {
    pub row: u16,
    pub column: u16,
  }

  impl Pos {
    pub fn new(row: u16, column: u16) -> Self {
      Pos { row, column }
    }
  }
}
