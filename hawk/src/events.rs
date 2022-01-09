use std::collections::HashMap;

use log::debug;

use crate::Context;
use keymaps::{Keymap, KeymapId};

mod keymaps;

// TEMP
struct FundamentalMode {}

impl FiletypeHandler for FundamentalMode {
  fn handle_key(&self, keypress: String) {
    println!("{}", keypress);
  }
}
// TEMP

pub struct EventListener {
  pub event_handler: EventHandler,
  pub keymap_handler: KeymapHandler,
}

impl EventListener {
  pub fn default() -> Self {
    let mut el = EventListener {
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
    };

    el.keymap_handler.keymaps.insert(
      KeymapId {
        id: "global".to_string(),
      },
      keymaps::create_default_global_keymap(),
    );

    el
  }
}

pub struct EventHandler {}

pub trait FiletypeHandler {
  fn handle_key(&self, keypress: String);
}

pub struct KeymapHandler {
  /// default bindings, often insert_self
  pub global: KeymapId,
  /// current filetype based on extension or shebang, used to
  /// dispatch keys to correct minor mode e.g files ending in .html
  /// can dispatch either html, css or js functions based on cursor
  /// position
  pub filetype_handler: Box<dyn FiletypeHandler>,
  pub minor: Vec<KeymapId>,
  pub keymaps: HashMap<KeymapId, Keymap>,
}

impl KeymapHandler {
  pub fn handle(&self, context: &mut Context, keypress: &str) {
    debug!("key press {}", &keypress);
    // self.filetype_handler.handle_key()

    if let Some(f) = self
      .keymaps
      .get(&KeymapId {
        id: "global".to_string(),
      })
      .and_then(|f| f.get(keypress))
    {
      f(context);
    }
  }

  pub fn register_keymap(&mut self, keymap_id: KeymapId, keymap: Keymap) {
    self.keymaps.insert(keymap_id, keymap);
  }
}

#[cfg(test)]
mod tests {
  use crate::{App, HawkEvent};

  use super::*;

  fn do_stuff(ctx: &mut Context) {
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
    keymap.insert("c".to_string(), Box::new(do_stuff));

    let fh = FH {};

    app
      .event_handler
      .keymap_handler
      .register_keymap(keymap_id, keymap);

    app.handle_event(HawkEvent::Key('c'));

    assert_eq!(app.editor.buffers.len(), 2);
  }
}
