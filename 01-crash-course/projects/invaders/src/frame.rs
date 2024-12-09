/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
// Project
use crate::{NUM_COLS, NUM_ROWS};
// Packages
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES & VARS
/* ========================================================================== */
pub type Frame = Vec<Vec<&'static str>>;

pub trait Drawable {
   fn draw(
      &self,
      frame: &mut Frame,
   );
}

/* ========================================================================== */
// DEFINING THE `FRAME` MODULE
/* ========================================================================== */
pub fn new_frame() -> Frame {
   let mut cols = Vec::with_capacity(NUM_COLS);

   for _ in 0..NUM_COLS {
      let mut col = Vec::with_capacity(NUM_ROWS);

      for _ in 0..NUM_ROWS {
         col.push(" ");
      }

      cols.push(col);
   }

   cols
}
