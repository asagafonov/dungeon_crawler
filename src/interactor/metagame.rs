use rust_i18n::t;
use crate::{engine::Engine, data::enumerables::Class};

pub(crate) struct MetagameController;

impl MetagameController {
  pub fn show_status(state: &Engine) {
    println!(
      "{} {}!",
      t!("metagame.role"),
      Self::class_as_string(&state.player.lock().unwrap().class),
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

  pub fn exit(state: &Engine) {
    state.progress.lock().unwrap().need_evac = true;
  }

  fn class_as_string(class: &Class) -> String {
    match class {
      Class::Warrior => t!("player.class.warrior"),
      Class::Mage => t!("player.class.mage"),
      Class::Rogue => t!("player.class.rogue"),
      Class::Any => String::new(),
    }
  }
}