/* ========================================================================== */
// ALL REQUIRED IMPORTS
/* ========================================================================== */
// Rust
use std::error::Error;
use std::time::{Duration, Instant};
use std::{io, thread};
// Project
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render::render;
use invaders::{exit_terminal, init_audio, init_terminal};
// Packages
use crossbeam::channel;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

/* ========================================================================== */
// HELPERS, INTERFACES & VARS
/* ========================================================================== */
/* ========================================================================== */
// DEFINING THE `MAIN THREAD` FILE
/* ========================================================================== */
fn main() -> Result<(), Box<dyn Error>> {
   // startup
   let mut audio = init_audio();
   init_terminal().expect("Initializing the terminal panicked!");

   // render loop in a separate thread
   let (render_tx, render_rx) = channel::unbounded();

   let render_handle = thread::spawn(move || {
      let mut last_frame = new_frame();
      let mut stdout = io::stdout();
      render(&mut stdout, &last_frame, &last_frame, true);

      'renderloop: loop {
         let curr_frame = match render_rx.recv() {
            Ok(x) => x,
            Err(_) => break 'renderloop,
         };

         render(&mut stdout, &last_frame, &curr_frame, false);
         last_frame = curr_frame;
      }
   });

   // game models
   let mut invaders = Invaders::new();
   let mut instant = Instant::now();
   let mut player = Player::new();

   // game loop
   'gameloop: loop {
      // per-frame init
      let delta = instant.elapsed();
      instant = Instant::now();
      let mut curr_frame = new_frame();

      // handling input
      while event::poll(Duration::default())? {
         if let Event::Key(key) = event::read()? {
            match key.code {
               KeyCode::Char(' ') | KeyCode::Enter => {
                  if player.shoot() {
                     audio.play("pew");
                  }
               },
               KeyCode::Esc | KeyCode::Char('q') => {
                  audio.play("lose");
                  break 'gameloop;
               },
               KeyCode::Left => player.move_left(),
               KeyCode::Right => player.move_right(),
               _ => (),
            }
         }
      }

      // updating timers
      player.update(delta);

      if invaders.update(delta) {
         audio.play("move");
      }

      // draw & render
      // =====[ v1 of rendering our models ]=====
      // invaders.draw(&mut curr_frame);
      // player.draw(&mut curr_frame);
      // =====[ v2 of rendering our models ]=====
      let drawables: Vec<&dyn Drawable> = vec![&invaders, &player];

      for drawable in drawables {
         drawable.draw(&mut curr_frame);
      }
      // the return value of this `.send` could be a `Result` or an `Error`
      // by storing it this way, we're effectively silently ignoring whatever happens
      let _ = render_tx.send(curr_frame);
      // an artificial delay to let the render loop keep up with the game loop
      thread::sleep(Duration::from_millis(1));
   }

   // cleanup
   drop(render_tx);
   render_handle.join().unwrap();
   audio.wait();
   exit_terminal().expect("Exiting the terminal panicked!");

   // finish
   Ok(())
}
