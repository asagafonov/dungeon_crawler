use rust_i18n::t;
use substring::Substring;
use crate::{
  engine::Engine,
  configurator::map::Map,
  data::types::{Content, MonsterLevel},
  shared::helpers::class_as_string,
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

        let mut progress = state.progress.lock().unwrap();
        progress.battle_mode = false;
        progress.monsters_killed += 1;
        progress.update_score();

        if let MonsterLevel::Boss = monster.level {
          progress.is_boss_defeated = true;
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
        let monster_damage = monster_attack_rate - defence_rate / 2;
        let monster_damage = if monster_damage <= 0 { 0 } else { monster_damage };
        println!("{}", t!("battle.monster_revenge", damage = monster_damage));
        println!("{}", t!("battle.block", damage = defence_rate / 2));
        state.player.lock().unwrap().health -= monster_damage;
      }
    }
  }

  pub fn retreat(state: &Engine) {
    let progress = &mut state.progress.lock().unwrap();
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
      terrain.visited = false;
    }

    println!("{}", t!("battle.you_have_fled"));
    let id = terrain.id.substring(0, terrain.id.len() - 1);
    progress.position = String::from(id);
    progress.battle_mode = false;
  }
}