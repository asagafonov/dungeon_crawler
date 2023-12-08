use rust_i18n::t;
use crate::{engine::Engine, data::enumerables::Content};

use super::movement::MovementController;

pub struct BattleController;

impl BattleController {
  pub fn fight(state: &Engine) {
    // if let Content::Monster(mut monster) = state.progress.lock().unwrap().monster {
    //   let attack_rate = &state.player.lock().unwrap().attack;
    //   monster.health -= attack_rate;
    //   println!("{}", t!("battle.hurt_monster", damage = attack_rate));
    //   println!("{}", t!("battle.monster.health_left", health = monster.health));

    //   if monster.health <= 0 {
    //     state.progress.lock().unwrap().battle_mode = false;
    //     println!("{}", t!("battle.monster_defeated"));
    //   }

    //   let monster_attack_rate = monster.attack;
    //   println!("{}", t!("battle.monster_revenge", damage = monster_attack_rate));
    //   state.player.lock().unwrap().health -= monster_attack_rate;
    // }
  }

  pub fn retreat(state: &Engine) {
    println!("{}", t!("battle.you_have_fled"));
    MovementController::go_backwards(state);
    state.progress.lock().unwrap().battle_mode = false;
  }
}