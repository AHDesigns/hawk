use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use hawk::logger::warn;

use hawk::Direction::*;
use hawk::HawkEvent::{self, *};

pub fn poll_user_input() -> Option<HawkEvent> {
  if event::poll(Duration::from_millis(16)).unwrap() {
    match event::read().unwrap() {
      Event::Mouse(_) => None,
      Event::Resize(w, h) => {
        warn!("screen resized {} {}", w, h);
        None
      }
      Event::Key(KeyEvent {
        modifiers: KeyModifiers::CONTROL,
        code: KeyCode::Char('c'),
      }) => Some(Quit),
      Event::Key(key) => match key.code {
        KeyCode::Enter => Some(Enter),
        KeyCode::Tab => Some(Slow),
        KeyCode::Backspace => Some(Delete),
        KeyCode::Char(k) => Some(Insert(k)),
        KeyCode::Up => Some(Move(Up)),
        KeyCode::Down => Some(Move(Down)),
        KeyCode::Right => Some(Move(Forward)),
        KeyCode::Left => Some(Move(Back)),
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
