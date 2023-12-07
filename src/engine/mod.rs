use std::io;
use std::sync::{Arc, Mutex};

use crate::data::enumerables::Player;
use crate::configurator::{map::Map, progress::Progress};
use crate::interactor::Interactor;

pub struct Engine {
  pub player: Arc<Mutex<Player>>,
  pub map: Arc<Mutex<Map>>,
  pub progress: Arc<Mutex<Progress>>,
}

impl Engine {
  pub fn new(map: Map, player: Player, progress: Progress) -> Engine {
    Engine {
      player: Arc::new(Mutex::new(player)),
      map: Arc::new(Mutex::new(map)),
      progress: Arc::new(Mutex::new(progress)),
    }
  }

  pub fn run(&self) {
    println!("{}", t!("game.started"));

    while !self.progress.lock().unwrap().is_boss_defeated & (self.player.lock().unwrap().health > 0) {
      // let (
      //   terrain_exists,
      //   current_terrain
      // ) = Map::find_by(&self.map.dungeon, &self.progress.position);

      // if !terrain_exists {
      //   panic!("out of bounds");
      // }

      let mut user_input = String::new();

      io::stdin()
          .read_line(&mut user_input)
          .expect(t!("errors.user_input").as_str());

      let command = user_input.trim();

      Interactor::execute(command, self);
    }
  }
}