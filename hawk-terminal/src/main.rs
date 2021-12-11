use crossterm::event::{self, Event, KeyCode};
use std::io::Error;

mod buffers;
mod logger;
mod ui;

use buffers::Buffer;
use logger::*;
use ui::Renderer;

/// This struct holds the state of the app.
struct App {
  buffers: Vec<Buffer>,
}

impl App {
  fn new() -> Self {
    App {
      buffers: Vec::new(),
    }
  }

  fn create_buffer(&mut self, name: String) {
    self.buffers.push(Buffer::new(name));
  }
}

fn main() -> Result<(), Error> {
  init_logger();

  info!("app starting");

  let mut screen = Renderer::new()?;

  let mut app = App::new();

  app.create_buffer("scratch".to_string());

  let active_buffer = 0;

  loop {
    let buff = app.buffers.get_mut(active_buffer).unwrap();

    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => {
          info!("quiting on char q");
          break;
        }
        KeyCode::Enter => {
          &buff.line_break();
        }
        KeyCode::Char(k) => {
          &buff.append_text(k.to_string());
        }
        KeyCode::Backspace => {
          debug!("backspace");
        }
        _ => {}
      }
    }

    screen.redraw(buff)?;
  }

  screen.cleanup()?;

  Ok(())
}
