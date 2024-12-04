/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::error::Error;
use std::io;
// Project
// Packages
use crossterm::cursor;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use rusty_audio::Audio;
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES & VARS
/* ========================================================================== */
pub mod frame;
pub mod render;
pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

/* ========================================================================== */
// DEFINING THE `MAIN` UTILS
/* ========================================================================== */
pub fn exit_terminal() -> Result<(), Box<dyn Error>> {
   let mut stdout = io::stdout();
   stdout.execute(cursor::Show)?;
   stdout.execute(LeaveAlternateScreen)?;
   terminal::disable_raw_mode()?;
   Ok(())
}

pub fn init_audio() -> Audio {
   let mut audio = Audio::new();
   // ADDING SFX
   audio.add("explode", "./assets/sounds/explode.wav");
   audio.add("lose", "./assets/sounds/lose.wav");
   audio.add("move", "./assets/sounds/move.wav");
   audio.add("pew", "./assets/sounds/pew.wav");
   audio.add("startup", "./assets/sounds/startup.wav");
   audio.add("win", "./assets/sounds/win.wav");
   // PLAYING SFX
   audio.play("startup");
   // audio.play("explode");
   // audio.play("lose");
   // audio.play("move");
   // audio.play("pew");
   // audio.play("win");
   audio
}

pub fn init_terminal() -> Result<(), Box<dyn Error>> {
   // setting up the terminal input gathering
   let mut stdout = io::stdout();
   terminal::enable_raw_mode()?;
   stdout.execute(EnterAlternateScreen)?;
   stdout.execute(cursor::Hide)?;
   Ok(())
}
