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
    state: &Engine,
  ) {
    if state.progress.lock().unwrap().battle_mode {
      match command {
        _ if command.is(t!("battle.fight")) => BattleController::fight(state),
        _ if command.is(t!("battle.retreat")) => BattleController::retreat(state),

        _ if command.is(t!("metagame.exit")) => MetagameController::exit(state),
        _ if command.is(t!("metagame.status")) => MetagameController::show_status(state),
        _ if command.is(t!("metagame.help_me")) => MetagameController::help_in_combat(),
        _ => MetagameController::do_nothing_in_combat(),
      }
    } else {
      match command {
        _ if command.is(t!("move.forward")) => MovementController::go_forward(state),
        _ if command.is(t!("move.left")) => MovementController::go_left(state),
        _ if command.is(t!("move.right")) => MovementController::go_right(state),
        _ if command.is(t!("move.back")) => MovementController::go_back(state),
        _ if command.is(t!("move.explore")) => MovementController::explore(state),
        _ if command.is(t!("move.entrance")) => MovementController::go_to_dungeon_entrance(state),

        _ if command.is(t!("metagame.rules")) => MetagameController::show_rules(),
        _ if command.is(t!("metagame.status")) => MetagameController::show_status(state),
        _ if command.is(t!("metagame.exit")) => MetagameController::exit(state),
        _ if command.is(t!("metagame.help_me")) => MetagameController::help(),
        _ => MetagameController::do_nothing(),
      }
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