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
    println!("{}", t!("game.delimeter"));
    println!();
    println!("{}", t!("metagame.description.goal"));
    println!("{}", t!("metagame.description.movement", explore = t!("move.explore")));
    println!("{}", t!("metagame.description.monsters"));
    println!("{}", t!("metagame.description.battle", fight = t!("battle.fight")));
    println!("{}", t!("metagame.description.retreat", retreat = t!("battle.retreat")));
    println!("{}", t!("metagame.description.traps"));
    println!("{}", t!("metagame.description.treasures"));
    println!("{}", t!("metagame.description.stuck", help = t!("metagame.help_me")));
    println!("{}", t!("metagame.description.good_luck"));
    println!();
    println!("{}", t!("game.delimeter"));
  }

  pub fn help() {
    println!("{}", t!("metagame.help.movement", explore = t!("move.explore")));
  }

  pub fn help_in_combat() {
    println!("{}", t!("metagame.help.battle", fight = t!("battle.fight"), retreat = t!("battle.retreat")));
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