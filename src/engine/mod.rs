use std::io;

use crate::data::enumerables::Player;
use crate::configurator::map::Map;
use crate::interactor::Interactor;

pub struct Engine {
  pub player: Player,
  pub map: Map,
}

impl Engine {
  pub fn new(map: Map, player: Player) -> Engine {
    Engine { player, map }
  }

  pub fn run(&mut self) {
    println!("{}", t!("game.started"));
    let is_boss_defeated = false;

    while !is_boss_defeated & (self.player.health > 0) {
      let mut user_input = String::new();

      io::stdin()
          .read_line(&mut user_input)
          .expect(t!("errors.user_input").as_str());

      let user_input = user_input.trim();

      Interactor::interpret(user_input, self);
    }
  }
}