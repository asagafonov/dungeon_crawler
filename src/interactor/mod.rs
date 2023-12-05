use rust_i18n::t;
use std::str;

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
  pub fn execute(
    command: &str,
    mut state: &mut Engine,
  ) {
    match command {
      // movement
      _ if command.is(t!("move.forward")) => MovementController::go_forward(&mut state),
      _ if command.is(t!("move.left")) => MovementController::go_left(&mut state),
      _ if command.is(t!("move.right")) => MovementController::go_right(&mut state),
      _ if command.is(t!("move.backwards")) => MovementController::go_backwards(&mut state),
      _ if command.is(t!("move.explore")) => MovementController::explore(&state),
      // battle
      _ if command.is(t!("battle.attack")) => BattleController::attack(&mut state),
      _ if command.is(t!("battle.retreat")) => BattleController::retreat(&mut state),
      // meta
      _ if command.is(t!("metagame.rules")) => MetagameController::show_rules(),
      _ if command.is(t!("metagame.status")) => MetagameController::show_status(&state),
      _ if command.is(t!("metagame.exit")) => MetagameController::exit(&state),
      _ => MetagameController::do_nothing(),
    }
  }
}

pub trait Identifier {
  fn is(&self, text: String) -> bool;
}

impl Identifier for str {
  fn is(&self, text: String) -> bool {
      text.eq(self)
  }
}