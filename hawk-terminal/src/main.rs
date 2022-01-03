use crossbeam::channel::{unbounded, Sender};
use crossterm::Result as UiResult;
use std::{thread, time::Duration};
use threadpool::ThreadPool;

extern crate num_cpus;

mod ui;
mod ux;

use hawk::logger::*;
use ui::Renderer;

use hawk::{
  App, Direction,
  HawkEvent::{self, *},
};

pub struct Cursor {
  pub row: u8,
  pub column: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logger();

  info!("app starting");

  let mut renderer = Renderer::new()?;

  let mut app = App::default(renderer.get_screen_size());

  app.create_buffer("scratch".to_string());

  let active_buffer: usize = 0;
  let mut cursor = Cursor { row: 0, column: 0 };

  let (worker_sender, event_reciever) = unbounded::<HawkEvent>();

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
      buff.line_break();
      renderer.redraw(buff, cursor)
    }
    Insert(k) => {
      cursor.column += 1;
      buff.append_text(k.to_string());
      renderer.redraw(buff, cursor)
    }
    Delete => {
      cursor.column -= 1;
      buff.remove_text(cursor.row);
      renderer.redraw(buff, cursor)
    }
    Move(direction) => {
      match direction {
        Direction::Up => {
          cursor.row -= 1;
        }
        Direction::Down => {
          cursor.row += 1;
        }
        Direction::Forward => {
          cursor.column += 1;
        }
        Direction::Back => {
          cursor.column -= 1;
        }
      }
      renderer.redraw(buff, cursor)
    }
    Resize((w, h)) => {
      app.display.resize(w, h);
      renderer.redraw(buff, cursor)
    }
    _ => {
      warn!("unhandled Hawk event: {:?}", e);

      Ok(())
    }
  }
}
