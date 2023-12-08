use rust_i18n::t;
use crate::{engine::Engine, data::enumerables::Content};

use super::movement::MovementController;

pub struct BattleController;

impl BattleController {
  pub fn attack(state: &Engine) {
    println!("attacking!");
  }

  pub fn retreat(state: &Engine) {
    println!("{}", t!("battle.you_have_fled"));
    MovementController::go_backwards(state);
  }
}