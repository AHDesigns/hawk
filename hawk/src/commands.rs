use crate::events::Context;

pub fn insert_newline(ctx: &mut Context) {
  ctx.editor.buffers.get_mut(0).unwrap().insert("\n");
}
