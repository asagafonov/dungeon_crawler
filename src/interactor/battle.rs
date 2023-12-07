use crate::{engine::Engine, data::enumerables::Content};

pub struct BattleController;

impl BattleController {
  pub fn attack(state: &Engine) {
    println!("attacking!");
  }

  pub fn retreat(state: &Engine) {
    println!("retreating");
  }
}