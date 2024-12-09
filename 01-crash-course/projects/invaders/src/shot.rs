/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::time::Duration;
// Project
use crate::frame::{Drawable, Frame};
// Packages
use rusty_time::Timer;
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES, VARS & SET UP
/* ========================================================================== */
pub struct Shot {
   pub x: usize,
   pub y: usize,
   pub exploding: bool,
   timer: Timer,
}

/* ========================================================================== */
// DEFINING THE `SHOT` MODULE
/* ========================================================================== */
impl Shot {
   pub fn new(
      x: usize,
      y: usize,
   ) -> Shot {
      Self {
         x,
         y,
         exploding: false,
         timer: Timer::new(Duration::from_millis(50)),
      }
   }

   pub fn update(
      &mut self,
      delta: Duration,
   ) {
      self.timer.tick(delta);

      if self.timer.finished() && !self.exploding {
         if self.y > 0 {
            self.y -= 1;
         }

         self.timer.reset();
      }
   }

   pub fn explode(&mut self) {
      self.exploding = true;
      self.timer = Timer::new(Duration::from_millis(250));
   }

   pub fn dead(&self) -> bool {
      (self.exploding && self.timer.finished()) || self.y == 0
   }
}

impl Drawable for Shot {
   fn draw(
      &self,
      frame: &mut Frame,
   ) {
      frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
   }
}
