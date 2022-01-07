use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use hawk::logger::warn;

use hawk::HawkEvent::{self, *};

pub fn poll_user_input() -> Option<HawkEvent> {
  if event::poll(Duration::from_millis(16)).unwrap() {
    match event::read().unwrap() {
      Event::Mouse(_) => None,
      Event::Resize(w, h) => {
        warn!("screen resized {} {}", w, h);
        Some(Resize((w, h)))
      }
      Event::Key(KeyEvent {
        modifiers: KeyModifiers::CONTROL,
        code: KeyCode::Char('c'),
      }) => Some(Quit),
      Event::Key(key) => match key.code {
        KeyCode::Enter => Some(Enter),
        KeyCode::Tab => Some(Slow),
        KeyCode::Backspace => Some(Delete),
        KeyCode::Char(k) => Some(Key(k)),
        KeyCode::Up => Some(Up),
        KeyCode::Down => Some(Down),
        KeyCode::Right => Some(Forward),
        KeyCode::Left => Some(Back),
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
