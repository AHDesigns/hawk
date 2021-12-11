use crate::buffers::Buffer;
use crossterm::cursor::{Hide, MoveDown, MoveTo, MoveToNextLine, Show};
use crossterm::style::Print;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnterAlternateScreen,
  LeaveAlternateScreen,
};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{self, stdout, Error, Stdout, Write};

pub struct Renderer {
  stdout: Stdout,
}

impl Renderer {
  pub fn new() -> Result<Self, Error> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    stdout
      .queue(EnterAlternateScreen)?
      .queue(DisableLineWrap)?
      .queue(Clear(ClearType::All))?
      .queue(MoveTo(0, 0))?
      .flush()?;

    Ok(Renderer { stdout })
  }

  pub fn cleanup(mut self) -> Result<(), Error> {
    self.stdout.execute(LeaveAlternateScreen)?.execute(Show)?;
    disable_raw_mode()?;

    Ok(())
  }

  pub fn redraw(&mut self, buff: &Buffer) -> Result<(), Error> {
    self
      .stdout
      .queue(Clear(ClearType::All))?
      .queue(MoveTo(0, 0))?;

    buff.text.iter().enumerate().for_each(|(i, line)| {
      self
        .stdout
        .queue(MoveTo(0, i as u16))
        .unwrap()
        .queue(Print(line))
        .unwrap();
    });

    self.stdout.flush()?;

    Ok(())
  }
}
