use crossbeam::channel::unbounded;

use threadpool::ThreadPool;

extern crate num_cpus;

mod ui;
mod ux;

use hawk::logger::*;
use ui::Renderer;

use hawk::{App, HawkEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logger();

  info!("app starting");

  let mut renderer = Renderer::new()?;

  let mut app = App::new(renderer.get_screen_size());

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
      Some(he) => {
        app.handle_event(he);
      }
      None => {
        if let Ok(he) = event_reciever.try_recv() {
          match he {
            HawkEvent::Quit => {
              info!("quiting");
              break;
            }
            _ => {
              app.handle_event(he);
            }
          };
        };
      }
    };

    renderer.redraw(&app)?;
  }

  renderer.cleanup()?;

  Ok(())
}
