use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
  self, disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnterAlternateScreen,
  LeaveAlternateScreen,
};
use crossterm::{ExecutableCommand, QueueableCommand};
use hawk::logger::{debug, info};
use hawk::App;
use std::io::{self, Error, Stdout, Write};

#[derive(Debug)]
struct FrameSize {
  columns: u16,
  rows: u16,
}

pub struct Renderer {
  stdout: Stdout,
  frame_size: FrameSize,
}

impl Renderer {
  pub fn new() -> Result<Self, Error> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    let (columns, rows) = terminal::size().expect("could not get terminal size");

    let frame_size = FrameSize { columns, rows };
    info!("frame size {:?}", frame_size);

    stdout
      .queue(EnterAlternateScreen)?
      .queue(DisableLineWrap)?
      .queue(Hide)?
      .queue(Clear(ClearType::All))?
      .queue(MoveTo(0, 0))?
      .flush()?;

    Ok(Renderer { stdout, frame_size })
  }

  pub fn cleanup(mut self) -> Result<(), Error> {
    self.stdout.execute(LeaveAlternateScreen)?.execute(Show)?;
    disable_raw_mode()?;

    Ok(())
  }

  pub fn redraw(&mut self, app: &App) -> Result<(), Error> {
    // starting by just repainting whole terminal for now, will improve later

    // clear terminal
    self
      .stdout
      .queue(Clear(ClearType::All))?
      .queue(MoveTo(0, 0))?;

    let buff = app.editor.buffers.get(0).unwrap();

    buff
      .content
      .text()
      .split('\n')
      .enumerate()
      .for_each(|(i, line)| {
        let end = {
          let size = line.len();
          let last_char = self.frame_size.columns as usize;
          if size > last_char {
            last_char
          } else {
            size
          }
        };

        let visible_line = &line[0..end];
        self
          .stdout
          .queue(MoveTo(0, i as u16))
          .unwrap()
          .queue(Print(visible_line))
          .unwrap();
      });

    self.stdout.flush()?;

    Ok(())
  }

  pub fn get_screen_size(&self) -> (u16, u16) {
    terminal::size().expect("could not get terminal size")
  }
}
