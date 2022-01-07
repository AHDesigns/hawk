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
        KeyCode::Char(k) => Some(Key(k)),
        KeyCode::Enter => Some(Enter),
        KeyCode::Up => Some(Up),
        KeyCode::Down => Some(Down),
        KeyCode::Right => Some(Right),
        KeyCode::Left => Some(Left),
        KeyCode::Backspace => Some(Backspace),
        KeyCode::Home => Some(Home),
        KeyCode::End => Some(End),
        KeyCode::PageUp => Some(PageUp),
        KeyCode::PageDown => Some(PageDown),
        KeyCode::Tab => Some(Tab),
        KeyCode::BackTab => Some(BackTab),
        KeyCode::Delete => Some(Delete),
        KeyCode::Insert => Some(Insert),
        KeyCode::F(n) => Some(F(n)),
        KeyCode::Null => panic!("the hell is this?? {:?}", key),
        KeyCode::Esc => Some(Esc),
      },
    }
  } else {
    None
  }
}
