use std::io;

use crate::{
  data::enumerables::Hero,
  configurator::map::Map,
  interactor::Interactor,
};

pub struct Engine {
  pub player: Hero,
  pub dungeon: Map,
}

impl Engine {
  pub fn run(&self) {
    let is_boss_defeated = false;

    while !is_boss_defeated & (self.player.health > 0) {
      let mut user_input = String::new();

      io::stdin()
          .read_line(&mut user_input)
          .expect(t!("errors.user_input").as_str());

      let user_input = user_input.trim();

      Interactor::interpret(user_input);
    }
  }
}