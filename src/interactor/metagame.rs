use rust_i18n::t;
use crate::engine::Engine;
use crate::shared::helpers::class_as_string;


pub struct MetagameController;

impl MetagameController {
  pub fn show_status(state: &Engine) {
    let player = state.player.lock().unwrap();

    println!(
      "{}",
      t!(
        "metagame.role",
        class = class_as_string(&player.class),
        health = &player.health,
        attack = &player.attack,
        defence = &player.defence,
      ),
     );
  }

  pub fn show_rules() {
    println!("{}", t!("metagame.goal"));
    println!("{}", t!("metagame.movement"));
    println!("{}", t!("metagame.monsters"));
    println!("{}", t!("metagame.treasures"));
    println!("{}", t!("metagame.traps"));
    println!("{}", t!("metagame.good_luck"));
  }

  pub fn do_nothing() {
    println!("{}", t!("metagame.idle"));
  }

  pub fn do_nothing_in_combat() {
    println!("{}", t!("metagame.idle_in_combat"));
  }

  pub fn exit(state: &Engine) {
    state.progress.lock().unwrap().need_evac = true;
  }
}