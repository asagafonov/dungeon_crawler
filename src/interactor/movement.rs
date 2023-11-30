use crate::engine::Engine;

pub struct MovementController;

impl MovementController {
  pub fn execute(input: &str, state: &mut Engine) {

    println!("Message from Movement: {}", input);
  }
}