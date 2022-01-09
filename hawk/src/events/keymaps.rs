use crate::commands;
use crate::Context;
use std::collections::HashMap;

pub type Keymap = HashMap<String, Box<fn(&mut Context)>>;

#[macro_export]
macro_rules! insert_self {
  ( $( $x:expr ),* ) => {
    {

      let mut keymap: Keymap = HashMap::new();

      $(
	keymap.insert($x.to_string(), Box::new(|ctx: &mut Context| {
	  ctx.editor.get_active_buffer().insert($x);
	}));
      )*
	keymap
    }
  };
}

#[derive(PartialEq, Eq, Hash)]
pub struct KeymapId {
  pub id: String,
}

pub fn create_default_global_keymap() -> Keymap {
  let mut keymap = insert_self!(
    "#", "1", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "0", "-", "=", "±", "!", "@", "£",
    "$", "%", "^", "&", "*", "(", ")", "_", "+", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p",
    "p", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "\\", ":", "[", "]", "{", "}", "`", "z",
    "x", "c", "v", "b", "n", "m", ",", ".", "/", "~", "?", "'", "\"", "|", "<", ">", "?", " "
  );

  keymap.insert("Enter".to_string(), Box::new(commands::insert_newline));

  keymap
}
