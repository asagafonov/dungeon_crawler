use crate::{engine::Engine, data::enumerables::Content};

pub struct BattleController;

impl BattleController {
  pub fn start_battle(monster: &Content, state: &mut Engine) {

  }

  pub fn execute(input: &str, state: &mut Engine) {

    println!("Message from battle: {}", input);
  }
}