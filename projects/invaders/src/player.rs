/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
// Project
use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
// Packages
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES, VARS & SET UP
/* ========================================================================== */
pub struct Player {
   x: usize,
   y: usize,
}

/* ========================================================================== */
// DEFINING THE `PLAYER` FILE
/* ========================================================================== */
impl Player {
   pub fn new() -> Player {
      Player {
         x: NUM_COLS / 2,
         y: NUM_ROWS - 1,
      }
   }

   pub fn move_left(&mut self) {
      if self.x > 0 {
         self.x -= 1;
      }
   }

   pub fn move_right(&mut self) {
      if self.x < NUM_COLS - 1 {
         self.x += 1;
      }
   }
}

impl Drawable for Player {
   fn draw(
      &self,
      frame: &mut Frame,
   ) {
      frame[self.x][self.y] = "A";
   }
}
