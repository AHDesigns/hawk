use crossbeam::channel::unbounded;
use crossterm::event::{self, Event, KeyCode};
use std::{io::Error, thread, time::Duration};
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
enum Direction {
  Up,
  Down,
  Forward,
  Back,
}

#[derive(Debug)]
enum HawkEvent {
  Quit,
  Insert(char),
  Enter,
  Delete,
  Move(Direction),
  Ping,
  Slow,
}

extern crate num_cpus;
use HawkEvent::*;

fn main() -> Result<(), Error> {
  init_logger();

  info!("app starting");

  let mut renderer = Renderer::new()?;

  let mut app = App::new();

  app.create_buffer("scratch".to_string());

  let active_buffer = 0;
  let mut cursor: (u8, u8) = (0, 0);

  let (user_sender, event_reciever) = unbounded::<HawkEvent>();
  let worker_sender = user_sender.clone();

  let n_workers = num_cpus::get() - 2;
  info!("workers: {}", n_workers);
  let pool = ThreadPool::new(n_workers);

  thread::spawn(move || {
    info!("spawned input handler thread");

    loop {
      match event::read() {
        Ok(Event::Key(key)) => match key.code {
          KeyCode::Enter => {
            user_sender.send(Enter).unwrap();
          }
          KeyCode::Char('s') => {
            user_sender.send(Slow).unwrap();
          }
          KeyCode::Char('q') => {
            user_sender.send(Quit).unwrap();
          }
          KeyCode::Char(k) => {
            user_sender.send(Insert(k)).unwrap();
          }
          KeyCode::Backspace => {
            user_sender.send(Delete).unwrap();
          }
          _ => {
            warn!("event was not handled");
          }
        },
        _ => {}
      }
    }
  });

  loop {
    let buff = app.buffers.get_mut(active_buffer).unwrap();

    match event_reciever.recv() {
      Ok(e) => match e {
        HawkEvent::Quit => {
          info!("quiting on char q");
          break;
        }
        Slow => {
          let sender = worker_sender.clone();

          pool.execute(move || {
            info!("spawned worker thread");

            thread::sleep(Duration::from_millis(5000));
            info!("done!");
            sender.send(Ping).unwrap();
          })
        }
        Enter => {
          cursor = {
            let (r, c) = cursor;
            (r + 1, c)
          };
          &buff.line_break();
        }
        Insert(k) => {
          cursor = {
            let (r, c) = cursor;
            (r, c + 1)
          };
          buff.append_text(k.to_string());
        }
        Delete => {
          &buff.remove_text(cursor.0);
        }
        _ => {
          warn!("unhandled Hawk event: {:?}", e)
        }
      },
      Err(_) => {
        panic!("thread error")
      }
    }

    renderer.redraw(buff)?;
  }

  renderer.cleanup()?;

  Ok(())
}
