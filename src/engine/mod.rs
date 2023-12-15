use std::io;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use rust_i18n::t;

use crate::configurator::{map::Map, player::Player, progress::Progress};
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
    println!("{}", t!("game.show_rules", rules = t!("metagame.rules")));

    while !self.progress.lock().unwrap().is_boss_defeated & (self.player.lock().unwrap().health > 0) {
      println!();
      if self.progress.lock().unwrap().need_evac {
        break;
      }

      let mut user_input = String::new();

      io::stdin()
          .read_line(&mut user_input)
          .expect(t!("errors.user_input").as_str());

      let command = user_input.trim();
      println!();

      Interactor::execute(command, self);
    }

    println!();
    if self.player.lock().unwrap().health <= 0 {
      println!("{}", t!("game.you_died"));
    } else if self.progress.lock().unwrap().is_boss_defeated {
      println!("{}", t!("game.you_won"));
    } else {
      println!("{}", t!("game.you_quit"));
    }

    self.progress.lock().unwrap().show_statistics();
    Self::pause();
  }

  fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "{}", t!("game.press_to_quit")).unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
}