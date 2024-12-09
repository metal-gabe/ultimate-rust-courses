/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::cmp::max;
use std::time::Duration;
// Project
use crate::{NUM_COLS, NUM_ROWS};
// Packages
use crate::frame::{Drawable, Frame};
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
pub struct Invader {
   pub x: usize,
   pub y: usize,
}

pub struct Invaders {
   pub army: Vec<Invader>,
   direction: i32,
   move_timer: Timer,
}

/* ========================================================================== */
// DEFINING THE `INVADERS` MODULE
/* ========================================================================== */
impl Invaders {
   pub fn new() -> Self {
      let mut army = Vec::new();

      for x in 0..NUM_COLS {
         for y in 0..NUM_ROWS {
            let x_bounds = (x > 1) && (x < NUM_COLS - 2);
            let y_bounds = (y > 0) && (y < 9);
            let coord_mods = (x % 2 == 0) && (y % 2 == 0);
            let should_add_invader = x_bounds && y_bounds && coord_mods;

            if should_add_invader {
               army.push(Invader { x, y });
            }
         }
      }

      Self {
         army,
         direction: 1,
         move_timer: Timer::new(Duration::from_millis(2_000)),
      }
   }

   pub fn update(
      &mut self,
      delta: Duration,
   ) -> bool {
      self.move_timer.tick(delta);

      // check the timer to see if we should move at all
      if self.move_timer.finished() {
         self.move_timer.reset();

         // is it time to move downwards?
         let mut downwards = false;

         if self.direction == -1 {
            let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);

            if min_x == 0 {
               self.direction = 1;
               downwards = true;
            }
         } else {
            let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);

            if max_x == NUM_COLS - 1 {
               self.direction = -1;
               downwards = true;
            }
         }

         if downwards {
            let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
            self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));

            for invader in self.army.iter_mut() {
               invader.y += 1;
            }
         } else {
            for invader in self.army.iter_mut() {
               invader.x = (invader.x as i32 + self.direction) as usize;
            }
         }

         return true;
      }

      false
   }

   pub fn is_all_killed(&self) -> bool {
      self.army.is_empty()
   }

   pub fn is_bottom_reached(&self) -> bool {
      self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
   }

   pub fn kill_invader_at(
      &mut self,
      x: usize,
      y: usize,
   ) -> bool {
      if let Some(idx) = self
         .army
         .iter()
         .position(|invader| (invader.x == x) && (invader.y == y))
      {
         self.army.remove(idx);
         true
      } else {
         false
      }
   }
}

impl Drawable for Invaders {
   fn draw(
      &self,
      frame: &mut Frame,
   ) {
      for invader in self.army.iter() {
         let is_in_second_half_of_timer =
            (self.move_timer.remaining().as_secs_f32() / self.move_timer.duration().as_secs_f32()) > 0.5;
         // display the alternating character model
         frame[invader.x][invader.y] = if is_in_second_half_of_timer { "x" } else { "+" };
      }
   }
}
