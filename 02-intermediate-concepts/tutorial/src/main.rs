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
   player.translation = Vec2::new(100.0, 100.0);
   player.rotation = std::f32::consts::FRAC_PI_2;
   player.scale = 0.5;
   player.collision = true;

   let car1 = game.add_sprite("car1", SpritePreset::RacingCarBlue);
   car1.translation = Vec2::new(300.0, 100.0);
   // car1.rotation = std::f32::consts::FRAC_PI_2;
   car1.scale = 0.5;
   car1.collision = true;

   let initial_state = GameState::default();
   game.add_logic(game_logic);
   game.run(initial_state);
}

fn game_logic(
   engine: &mut Engine,
   state: &mut GameState,
) {
   for event in engine.collision_events.drain(..) {
      if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
         // remove the sprite the player collided with
         for label in [event.pair.0, event.pair.1] {
            if label != "player" {
               engine.sprites.remove(&label);
            }
         }

         state.current_score += 1;
         println!("Score: {}", state.current_score);
      }
   }

   let player = engine.sprites.get_mut("player").unwrap();

   // player movement
   const MOVEMENT_SPEED: f32 = 100.0;

   if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
      player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
   }

   if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
      player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
   }

   if engine.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
      player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
   }

   if engine.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
      player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
   }
}
