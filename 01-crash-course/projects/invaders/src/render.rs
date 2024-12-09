/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::io::{Stdout, Write};
// Project
use crate::frame::Frame;
// Packages
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES, VARS & SET UP
/* ========================================================================== */
/* ========================================================================== */
// DEFINING THE `RENDER` MODULE
/* ========================================================================== */
pub fn render(
   stdout: &mut Stdout,
   last_frame: &Frame,
   curr_frame: &Frame,
   force: bool,
) {
   if force {
      stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
      stdout.queue(Clear(ClearType::All)).unwrap();
      stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
   }

   for (x, col) in curr_frame.iter().enumerate() {
      for (y, s) in col.iter().enumerate() {
         // `s` is the actual character value of the cell
         if *s != last_frame[x][y] || force {
            stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
            print!("{s}");
         }
      }

      stdout.flush().unwrap();
   }
}
