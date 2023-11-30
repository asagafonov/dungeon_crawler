use rust_i18n::t;

pub mod battle;
pub mod movement;
pub mod metagame;

use crate::engine::Engine;

use crate::interactor::{
  battle::BattleController,
  movement::MovementController,
  metagame::MetagameController,
};

pub struct Interactor;

impl Interactor {
  pub fn interpret(user_input: &str, state: &mut Engine) {
    match user_input {
      input if Interactor::is_movement(user_input) => MovementController::execute(input, state),
      input if Interactor::is_battle(user_input) => BattleController::execute(input, state),
      input if Interactor::is_metagame(user_input) => MetagameController::execute(input, state),
      _ => println!("{}", t!("input.inexistant"))
    }
  }

  fn is_movement(input: &str) -> bool {
    match input {
      _ if t!("move.forward").eq(input) => true,
      _ if t!("move.left").eq(input) => true,
      _ if t!("move.right").eq(input) => true,
      _ if t!("move.backwards").eq(input) => true,
      _ if t!("move.explore").eq(input) => true,
      _ => false,
    }
  }

  fn is_battle(input: &str) -> bool {
    match input {
      _ if t!("battle.attack").eq(input) => true,
      _ if t!("battle.retreat").eq(input) => true,
      _ => false,
    }
  }

  fn is_metagame(input: &str) -> bool {
    match input {
      _ if t!("metagame.rules").eq(input) => true,
      _ if t!("metagame.actions").eq(input) => true,
      _ => false,
    }
  }
}