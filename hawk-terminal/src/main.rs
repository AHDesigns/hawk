use crossbeam::channel::{unbounded, Sender};
use crossterm::Result as UiResult;
use std::{thread, time::Duration};
use threadpool::ThreadPool;

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

#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Forward,
  Back,
}

#[derive(Debug)]
pub enum HawkEvent {
  Quit,
  Insert(char),
  Enter,
  Delete,
  Move(Direction),
  Ping,
  Slow,
}

mod ux {
  use std::time::Duration;

  use crossterm::event::{self, Event, KeyCode};
  use log::warn;

  use crate::HawkEvent::{self, *};

  pub fn poll_user_input() -> Option<HawkEvent> {
    if event::poll(Duration::from_millis(16)).unwrap() {
      match event::read().unwrap() {
        Event::Mouse(_) => None,
        Event::Resize(w, h) => {
          warn!("screen resized {} {}", w, h);
          None
        }
        Event::Key(key) => match key.code {
          KeyCode::Enter => Some(Enter),
          KeyCode::Char('s') => Some(Slow),
          KeyCode::Char('q') => Some(Quit),
          KeyCode::Char(k) => Some(Insert(k)),
          KeyCode::Backspace => Some(Delete),
          _ => {
            warn!("key was not handled {:?}", key);
            None
          }
        },
      }
    } else {
      None
    }
  }
}

struct Cursor {
  pub row: u8,
  pub column: u8,
}

extern crate num_cpus;
use HawkEvent::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logger();

  info!("app starting");

  let mut renderer = Renderer::new()?;

  let mut app = App::new();

  app.create_buffer("scratch".to_string());

  let active_buffer: usize = 0;
  let mut cursor = Cursor { row: 0, column: 0 };

  let (user_sender, event_reciever) = unbounded::<HawkEvent>();
  let worker_sender = user_sender.clone();

  let n_workers = num_cpus::get() - 1;

  info!("workers: {}", n_workers);

  let pool = ThreadPool::new(n_workers);

  loop {
    let e = ux::poll_user_input();

    match e {
      Some(HawkEvent::Quit) => {
        info!("quiting");
        break;
      }
      Some(he) => handle_event(
        &mut app,
        &pool,
        &mut renderer,
        he,
        active_buffer,
        &worker_sender,
        &mut cursor,
      )?,
      None => {
        if let Ok(he) = event_reciever.try_recv() {
          match he {
            HawkEvent::Quit => {
              info!("quiting");
              break;
            }
            _ => handle_event(
              &mut app,
              &pool,
              &mut renderer,
              he,
              active_buffer,
              &worker_sender,
              &mut cursor,
            )?,
          };
        };
      }
    };
  }

  renderer.cleanup()?;

  Ok(())
}

fn handle_event(
  app: &mut App,
  pool: &ThreadPool,
  renderer: &mut Renderer,
  e: HawkEvent,
  active_buffer: usize,
  worker_sender: &Sender<HawkEvent>,
  cursor: &mut Cursor,
) -> UiResult<()> {
  let buff = app.buffers.get_mut(active_buffer).unwrap();

  match e {
    Slow => {
      let sender = worker_sender.clone();

      pool.execute(move || {
        info!("spawned worker thread");

        thread::sleep(Duration::from_millis(5000));
        info!("done!");
        sender.send(Ping).unwrap();
      });

      Ok(())
    }
    Enter => {
      cursor.row += 1;
      &buff.line_break();
      renderer.redraw(buff)
    }
    Insert(k) => {
      cursor.column += 1;
      buff.append_text(k.to_string());
      renderer.redraw(buff)
    }
    Delete => {
      &buff.remove_text(cursor.row);
      renderer.redraw(buff)
    }
    _ => {
      warn!("unhandled Hawk event: {:?}", e);

      Ok(())
    }
  }
}
