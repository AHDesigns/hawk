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
      todo!();
    }
  }
}

use editor::Editor;

/// Holds the state of the application, handling the editor loop and
/// deligating to the editor appropriately
pub struct App<'a> {
  pub event_handler: EventListener<'a>,
  pub editor: Editor,
  pub display: Display,
}

impl<'a> App<'a> {
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
    let ctx = Context {
      editor: &mut self.editor,
    };

    if let HawkEvent::Key(k) = e {
      let keypress = &k.to_string();
      self.event_handler.keymap_handler.handle(ctx, keypress);
    }
  }

  // XXX: having a break now, don't know enough about lifetimes, and just pushing things around here
  pub fn register_keymap(&'a mut self, keymap_id: KeymapId, keymap: Keymap<'a>) -> &'a mut Self {
    self
      .event_handler
      .keymap_handler
      .register_keymap(keymap_id, keymap);

    &mut self
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

// mod commands_as_enum {
//   use crate::{App, Direction};

//   fn open_file(app: &mut App, file: &str) -> anyhow::Result<()> {
//     let buff = std::fs::read_to_string(std::path::Path::new(file))?;
//     print!("{}", buff);
//     app.create_buffer();
//     Ok(())
//   }

//   fn split_window(app: &mut App, _: &Direction) -> anyhow::Result<()> {
//     // let buffer_ref = app.display.get_active_window_buffer
//     // app.display.split_window(buffer_ref);
//     Ok(())
//   }

//   enum Command<'a> {
//     OpenFile(&'a str),
//     SplitWindow(Direction),
//   }

//   impl Command<'_> {
//     fn doc(&self) -> &str {
//       match self {
//         Command::OpenFile(_) => "Open file with name",
//         Command::SplitWindow(_) => "Split window in given direction",
//       }
//     }

//     fn run(&self, app: &mut App) -> anyhow::Result<()> {
//       match self {
//         Command::OpenFile(f) => open_file(app, f),
//         Command::SplitWindow(d) => split_window(app, d),
//       }
//     }
//   }
// }

// mod commands_as_trait {
//   use crate::{buffers::BufferId, App, Direction};

//   pub trait Command {
//     fn doc(&self) -> &'static str;
//     fn run(&self, app: &mut App) -> anyhow::Result<()>;
//   }

//   pub struct SplitWindow {
//     buffer: Option<BufferId>,
//     dir: Direction,
//   }

//   impl Command for SplitWindow {
//     fn doc(&self) -> &'static str {
//       "Split window in direction"
//     }

//     fn run(&self, _: &mut App) -> anyhow::Result<()> {
//       println!("{} {:?}", &self.buffer.unwrap_or(3), &self.dir);
//       // do stuff with app
//       Ok(())
//     }
//   }
// }
