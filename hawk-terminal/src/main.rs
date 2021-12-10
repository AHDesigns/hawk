use crossterm::cursor::{Hide, MoveDown, MoveTo, MoveToNextLine, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::Print;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::fs::File;
use std::io::{self, stdout, Error, Write};
use std::path::Path;

mod buffers {
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

    pub fn line_break(&mut self) {
      self.text.push("".to_string())
    }
  }
}

mod debugger {
  use std::io::Write;

  pub struct Debugger<T: Write> {
    transport: T,
  }

  impl<T: Write> Debugger<T>
  {
    pub fn new(transport: T) -> Self {
      Debugger { transport }
    }
    pub fn info(&mut self, msg: &[u8]) {
      &self.transport.write_all(msg);
    }
  }
}

use buffers::Buffer;
use debugger::Debugger;

/// This struct holds the state of the app.
struct App<T: Write> {
  buffers: Vec<Buffer>,
  debugger: Debugger<T>,
}

impl<T: Write> App<T> {
  fn new(transport: T) -> Self {
    App {
      buffers: Vec::new(),
      debugger: Debugger::new(transport),
    }
  }

  fn create_buffer(&mut self, name: String) {
    self.buffers.push(Buffer::new(name));
  }
}

#[cfg(debug_assertions)]
mod stuff {
  pub fn cfgtester() {
    println!("goodbye!")
  }
}

#[cfg(not(debug_assertions))]
mod stuff {
  pub fn cfgtester() {
    println!("hello!")
  }
}

fn main() -> Result<(), Error> {
  let mut stdout = io::stdout();

  enable_raw_mode()?;

  stdout
    .queue(EnterAlternateScreen)?
    .queue(Clear(ClearType::All))?
    .queue(MoveTo(0, 0))?
    .queue(Hide)?
    .flush()?;

  let debug_file = File::create("log.txt").expect("could not create debug log file");

  let mut app = App::new(debug_file);

  app.debugger.info("app started!".as_bytes());

  app.create_buffer("scratch".to_string());

  let active_buffer = 0;

  loop {
    let buff = app.buffers.get_mut(active_buffer).unwrap();

    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => {
          break;
        }
        // KeyCode::Char('l') => match hawk::buffers::open_buffer(Path::new("Cargo.toml")) {
        //   Ok(buff) => {
        //     for line in buff.lines {
        //       stdout.queue(Print(line))?.queue(MoveToNextLine(1))?;
        //     }
        //   }
        //   Err(_) => {}
        // },
        KeyCode::Enter => {
          &buff.line_break();
        }
        KeyCode::Char(k) => {
          &buff.append_text(k.to_string());
        }
        // KeyCode::Down => {
        //   y = y + 1;
        // }
        // KeyCode::Up => {
        //   if y > 0 {
        //     y = y - 1;
        //   }
        // }
        // KeyCode::Right => {
        //   x = x + 1;
        // }
        // KeyCode::Left => {
        //   x = x - 1;
        // }
        _ => {}
      }
    }

    stdout.queue(Clear(ClearType::All))?.queue(MoveTo(0, 0))?;

    buff.text.iter().enumerate().for_each(|(i, line)| {
      stdout
        .queue(MoveTo(0, i as u16))
        .unwrap()
        .queue(Print(line))
        .unwrap();
    });

    stdout.queue(Hide)?.flush()?;
  }

  stdout.execute(LeaveAlternateScreen)?.execute(Show)?;
  disable_raw_mode()?;

  stuff::cfgtester();
  Ok(())
}

// fn highlight_toml(lines: Vec<&String>) -> Vec<u8> {

// }
