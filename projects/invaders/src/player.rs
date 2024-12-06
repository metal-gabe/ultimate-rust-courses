/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::time::Duration;
// Project
use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
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
   shots: Vec<Shot>,
}

/* ========================================================================== */
// DEFINING THE `PLAYER` MODULE
/* ========================================================================== */
impl Player {
   pub fn new() -> Player {
      Player {
         x: NUM_COLS / 2,
         y: NUM_ROWS - 1,
         shots: Vec::new(),
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

   pub fn shoot(&mut self) -> bool {
      if self.shots.len() < 3 {
         let shot = Shot::new(self.x, self.y - 1);
         self.shots.push(shot);
         true
      } else {
         false
      }
   }

   pub fn update(
      &mut self,
      delta: Duration,
   ) {
      for shot in self.shots.iter_mut() {
         shot.update(delta);
      }

      self.shots.retain(|s| !s.dead());
   }
}

impl Drawable for Player {
   fn draw(
      &self,
      frame: &mut Frame,
   ) {
      frame[self.x][self.y] = "A";

      for shot in self.shots.iter() {
         shot.draw(frame);
      }
   }
}
