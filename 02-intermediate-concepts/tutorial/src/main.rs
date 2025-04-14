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
         enemy_labels: Vec::new(),
         high_score: 0,
         spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
      }
   }
}

fn main() {
   let mut game = Game::new();
   let player = game.add_sprite("player", SpritePreset::RacingCarRed);
   player.translation = Vec2::new(200.0, 100.0);
   player.rotation = std::f32::consts::FRAC_PI_2;
   player.scale = 0.5;
   let initial_state = GameState::default();
   game.add_logic(game_logic);
   game.run(initial_state);
}

fn game_logic(engine: &mut Engine, state: &mut GameState) {
   // state.current_score += 1;
   // println!("Score: {}", state.current_score);
}
