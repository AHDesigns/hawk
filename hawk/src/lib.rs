mod buffers;
mod commands;
mod display;
mod editor;
pub mod events;
pub mod logger;
mod util;

use display::Display;
use editor::Editor;
use events::*;

/// Holds the state of the application, handling the editor loop and
/// deligating to the editor appropriately
pub struct App {
  pub event_handler: EventListener,
  pub editor: Editor,
  pub display: Display,
}

impl App {
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
    let mut ctx = Context {
      editor: &mut self.editor,
    };

    // again, yuk, will be clever later
    let keypress = match e {
      HawkEvent::Key(k) => String::from(k),
      HawkEvent::Backspace => "Backspace".to_string(),
      HawkEvent::Enter => "Enter".to_string(),
      HawkEvent::Left => "Left".to_string(),
      HawkEvent::Right => "Right".to_string(),
      HawkEvent::Up => "Up".to_string(),
      HawkEvent::Down => "Down".to_string(),
      HawkEvent::Tab => "Tab".to_string(),
      HawkEvent::Delete => "Delete".to_string(),
      HawkEvent::Esc => "Esc".to_string(),
      _ => panic!("no handler for event: {:?}", e),
    };

    self
      .event_handler
      .keymap_handler
      .handle(&mut ctx, &keypress);
  }
}

// TODO: copied a load from crossterm, can look into parsing or some From trait later
#[derive(Debug)]
pub enum HawkEvent {
  Quit,
  Key(char),
  Resize((u16, u16)),
  /// Backspace key.
  Backspace,
  /// Enter key.
  Enter,
  /// Left arrow key.
  Left,
  /// Right arrow key.
  Right,
  /// Up arrow key.
  Up,
  /// Down arrow key.
  Down,
  /// Home key.
  Home,
  /// End key.
  End,
  /// Page up key.
  PageUp,
  /// Page dow key.
  PageDown,
  /// Tab key.
  Tab,
  /// Shift + Tab key.
  BackTab,
  /// Delete key.
  Delete,
  /// Insert key.
  Insert,
  /// F key.
  ///
  /// `KeyCode::F(1)` represents F1 key, etc.
  F(u8),
  /// Null.
  Null,
  /// Escape key.
  Esc,
}
