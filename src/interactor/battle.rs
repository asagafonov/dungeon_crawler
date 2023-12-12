use rust_i18n::t;
use crate::{
  engine::Engine,
  configurator::map::Map,
  data::types::{Content, MonsterLevel},
  shared::helpers::class_as_string,
  interactor::movement::MovementController,
};

pub struct BattleController;

impl BattleController {
  pub fn fight(state: &Engine) {
    let progress = state.progress.lock().unwrap();
    let position = &progress.position;
    let dungeon = &mut state.map.lock().unwrap().dungeon;

    let terrain = Map::find(
      dungeon,
      position.as_str(),
      0
    );

    drop(progress);
    let content = &mut terrain.content;
    let attack_rate = state.player.lock().unwrap().attack;
    let defence_rate = state.player.lock().unwrap().defence;

    if let Content::Monster(monster) = content {
      monster.health -= attack_rate;
      println!("{}", t!("battle.hurt_monster", damage = attack_rate));
      println!("{}", t!("battle.monster.health_left", health = monster.health));

      if monster.health <= 0 {
        println!("{}", t!("battle.monster_defeated"));
        state.progress.lock().unwrap().battle_mode = false;

        if let MonsterLevel::Boss = monster.level {
          state.progress.lock().unwrap().is_boss_defeated = true;
        } else {
          println!("{}", t!("battle.loot_discovered"));
          let treasure = &monster.loot;
          state.player.lock().unwrap().equip(treasure);
        }
      } else {
        let monster_attack_rate: i8 = if class_as_string(&monster.hates) == class_as_string(&state.player.lock().unwrap().class) {
          monster.attack + 2
        } else {
          monster.attack
        };
        println!("{}", t!("battle.monster_revenge", damage = monster_attack_rate));
        state.player.lock().unwrap().health -= monster_attack_rate - defence_rate / 2;
      }
    }
  }

  pub fn retreat(state: &Engine) {
    let progress = state.progress.lock().unwrap();
    let position = &progress.position;
    let dungeon = &mut state.map.lock().unwrap().dungeon;

    let terrain = Map::find(
      dungeon,
      position.as_str(),
      0
    );

    let content = &mut terrain.content;

    if let Content::Monster(monster) = content {
      monster.hates = state.player.lock().unwrap().class.clone();
      monster.health += 10;
    }

    println!("{}", t!("battle.you_have_fled"));
    MovementController::go_back(state);
    state.progress.lock().unwrap().battle_mode = false;
  }
}