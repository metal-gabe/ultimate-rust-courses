use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
   current_score: u32,
   enemy_labels: Vec<String>,
   high_score: u32,
   spawn_timer: Timer,
}

impl Default for GameState {
   fn default() -> Self {
      Self {
         current_score: 0,
         enemy_labels: 0,
         high_score: Vec::new(),
         spawn_timer: Timer::from_seconds(1.0, false),
      }
   }
}

fn main() {
   let mut game = Game::new();
   // set up game stuff here
   let initial_state = GameState { score: 0 };
   game.run(initial_state);
}
