use std::collections::HashMap;

use log::{debug, info};

use crate::editor::Editor;

// TEMP
struct FundamentalMode {}

impl FiletypeHandler for FundamentalMode {
  fn handle_key(&self, keypress: String) {
    println!("{}", keypress);
  }
}
// TEMP

pub struct Context<'a> {
  pub editor: &'a mut Editor,
}

pub struct EventListener<'a> {
  pub event_handler: EventHandler,
  pub keymap_handler: KeymapHandler<'a>,
}

impl<'a> EventListener<'a> {
  pub fn default() -> Self {
    EventListener {
      event_handler: EventHandler {},
      keymap_handler: KeymapHandler {
        global: KeymapId {
          id: "global".to_owned(),
        },
        filetype_handler: Box::new(FundamentalMode {}),
        minor: vec![KeymapId {
          id: "foo".to_owned(),
        }],
        keymaps: HashMap::new(),
      },
    }
  }
}

pub struct EventHandler {}

pub trait FiletypeHandler {
  fn handle_key(&self, keypress: String);
}

pub type Keymap<'a> = HashMap<&'static str, &'a dyn Fn(Context)>;

pub struct KeymapHandler<'a> {
  /// default bindings, often insert_self
  pub global: KeymapId,
  /// current filetype based on extension or shebang, used to
  /// dispatch keys to correct minor mode e.g files ending in .html
  /// can dispatch either html, css or js functions based on cursor
  /// position
  pub filetype_handler: Box<dyn FiletypeHandler>,
  pub minor: Vec<KeymapId>,
  pub keymaps: HashMap<KeymapId, Keymap<'a>>,
}

impl<'a> KeymapHandler<'a> {
  pub fn handle(&'a self, context: Context, keypress: &str) {
    debug!("key press {}", &keypress);
    // self.filetype_handler.handle_key()

    if let Some(f) = self
      .keymaps
      .get(&KeymapId {
        id: "foo".to_string(),
      })
      .and_then(|f| f.get(keypress))
    {
      f(context);
    }
  }

  pub fn register_keymap(&'a mut self, keymap_id: KeymapId, keymap: Keymap<'a>) -> &Self {
    self.keymaps.insert(keymap_id, keymap);
    self
  }
}

#[derive(PartialEq, Eq, Hash)]
pub struct KeymapId {
  pub id: String,
}

#[cfg(test)]
mod tests {
  use crate::{App, HawkEvent};

  use super::*;

  fn do_stuff(ctx: Context) {
    ctx.editor.create_buffer();
  }

  struct FH {}
  impl FiletypeHandler for FH {
    fn handle_key(&self, keypress: String) {
      println!("{}", keypress);
    }
  }

  #[test]
  fn test_handler() {
    // just a dummy test to see if the keymap handler can be bound
    // to certain functions
    let mut app = App::default();

    let keymap_id = KeymapId {
      id: "foo".to_string(),
    };

    let mut keymap: Keymap = HashMap::new();
    keymap.insert("hi", &do_stuff);

    let fh = FH {};

    app.register_keymap(keymap_id, keymap);

    app.handle_event(HawkEvent::Key('c'));

    assert_eq!(app.editor.buffers.len(), 2);
  }
}
