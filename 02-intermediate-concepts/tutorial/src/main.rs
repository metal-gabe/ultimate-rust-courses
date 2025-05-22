use rand::Rng;
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
   enemy_index: i32,
   high_score: u32,
   score: u32,
   spawn_timer: Timer,
}

impl Default for GameState {
   fn default() -> Self {
      Self {
         enemy_index: 0,
         high_score: 0,
         score: 0,
         spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
      }
   }
}

fn main() {
   let mut game = Game::new();

   game.window_settings(Window {
      resizable: false,
      resolution: WindowResolution::new(800.0, 600.0),
      title: "Rusty Engine Tutorial".to_string(),
      ..Default::default()
   });

   game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

   let player = game.add_sprite("player", SpritePreset::RacingCarRed);
   player.translation = Vec2::new(100.0, 100.0);
   player.rotation = std::f32::consts::FRAC_PI_2;
   player.scale = 0.5;
   player.collision = true;

   let score_text = game.add_text("score", "Score: 0");
   score_text.translation = Vec2::new(520.0, 320.0);
   let high_score_text = game.add_text("high_score", "High Score: 0");
   high_score_text.translation = Vec2::new(-520.0, 320.0);

   let initial_state = GameState::default();
   game.add_logic(game_logic);
   game.run(initial_state);
}

fn game_logic(
   engine: &mut Engine,
   game_state: &mut GameState,
) {
   // quit if `q` is pressed
   if engine.keyboard_state.just_pressed(KeyCode::Q) {
      engine.should_exit = true;
   }

   // hover the score text up & down based on time since the game started
   let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
   // keep the score texts near the edges of the screen
   let score = engine.texts.get_mut("score").unwrap();
   score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
   score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
   let high_score = engine.texts.get_mut("high_score").unwrap();
   high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
   high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

   for event in engine.collision_events.drain(..) {
      if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
         // remove the sprite the player collided with
         for label in [event.pair.0, event.pair.1] {
            if label != "player" {
               engine.sprites.remove(&label);
            }
         }

         game_state.score += 1;

         let score = engine.texts.get_mut("score").unwrap();
         score.value = format!("Score: {}", game_state.score);

         if game_state.score > game_state.high_score {
            game_state.high_score = game_state.score;
            let high_score = engine.texts.get_mut("high_score").unwrap();
            high_score.value = format!("High Score: {}", game_state.high_score);
         }

         engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.25);
      }
   }

   let player = engine.sprites.get_mut("player").unwrap();

   // handle player movement
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

   // handle mouse input to spawn enemies
   if engine.mouse_state.just_pressed(MouseButton::Left) {
      if let Some(mouse_location) = engine.mouse_state.location() {
         let label = format!("enemy{}", game_state.enemy_index);
         game_state.enemy_index += 1;
         let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarBlue);
         enemy.translation = mouse_location;
         enemy.collision = true;
      }
   }

   if game_state.spawn_timer.tick(engine.delta).just_finished() {
      let label = format!("enemy{}", game_state.enemy_index);
      let mut rng = rand::rng();
      game_state.enemy_index += 1;
      let enemy = engine.add_sprite(label.clone(), SpritePreset::RacingCarBlue);
      enemy.translation.x = rng.random_range(-550.0..550.0);
      enemy.translation.y = rng.random_range(-325.0..325.0);
      enemy.collision = true;
   }

   // reset score
   if engine.keyboard_state.just_pressed(KeyCode::R) {
      game_state.score = 0;
      let score = engine.texts.get_mut("score").unwrap();
      score.value = "Score: 0".to_string();
   }
}
