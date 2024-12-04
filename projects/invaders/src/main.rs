// Rust
use std::error::Error;
// Project
use invaders::{exit_terminal, init_audio, init_terminal};
// Packages
// Context / Stores / Routers
// Components / Classes / Controllers / Services
// Assets / Constants
// Interfaces / Models / Types
// Utils / Decorators / Methods / Mocks
// Styles

fn main() -> Result<(), Box<dyn Error>> {
    // startup
    let audio = init_audio();
    init_terminal().expect("TODO: panic message");
    // cleanup
    audio.wait();
    exit_terminal().expect("TODO: panic message");
    // finish
    Ok(())
}
