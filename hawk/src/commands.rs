use crate::Context;

pub fn insert_newline(ctx: &mut Context) {
  ctx.editor.get_active_buffer().insert("\n");
}
