use crate::buffers::{contents::Content, Buffer, BufferId};

struct IdGenerator {
  last: BufferId,
}

impl IdGenerator {
  pub fn default() -> Self {
    IdGenerator { last: 0 }
  }

  pub fn next_id(&mut self) -> BufferId {
    let last = self.last;
    self.last += 1;
    last
  }
}

/// This is what the user is interacting with, and does most of the
/// work
pub struct Editor {
  buff_id: IdGenerator,
  pub buffers: Vec<Buffer>,
}

impl Editor {
  pub fn new() -> Self {
    let mut buff_id = IdGenerator::default();

    let buffers = vec![Buffer::new(
      buff_id.next_id(),
      "*scratch*".to_string(),
      Content::default(),
    )];

    Self { buff_id, buffers }
  }

  pub fn create_buffer(&mut self) {
    self.buffers.push(Buffer::new(
      self.buff_id.next_id(),
      "oijwef".to_string(),
      Content::default(),
    ));
  }

  pub fn get_active_buffer(&mut self) -> &mut Buffer {
    self.buffers.get_mut(0).unwrap()
  }
}
