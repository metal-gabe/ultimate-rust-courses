/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
// Project
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
pub struct Invader {
   pub x: usize,
   pub y: usize,
}

pub struct Invaders {
   pub army: Vec<Invader>,
   pub move_timer: Timer,
}

/* ========================================================================== */
// DEFINING THE `INVADERS` MODULE
/* ========================================================================== */
