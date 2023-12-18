use rust_i18n::t;
use substring::Substring;
use crate::{
  engine::Engine,
  configurator::map::{Map, Terrain},
  data::types::{Content, TrapClass, MonsterLevel},
  shared::helpers::class_as_string,
};

pub struct Directions {
  forward: bool,
  left: bool,
  right: bool,
  back: bool,
  forward_route_index: usize,
}

impl Directions {
  pub fn new() -> Directions {
    Directions {
      forward: false,
      left: false,
      right: false,
      back: true,
      forward_route_index: 0,
    }
  }
}

pub struct MovementController;

impl MovementController {
  pub fn go_forward(state: &Engine) {
    let dungeon = &mut state.map.lock().unwrap().dungeon;
    let terrain = Map::find(
      dungeon,
      &state.progress.lock().unwrap().position.as_str(),
      0
    );
    let directions = Self::available_directions(terrain);

    if directions.forward {
      println!("{}", t!("move.success"));
      let next_terrain = &mut terrain.children[directions.forward_route_index];
      let id = next_terrain.id.clone();

      Self::change_position(state, next_terrain, id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_left(state: &Engine) {
    println!("{}", t!("move.success"));
    let dungeon = &mut state.map.lock().unwrap().dungeon;
    let terrain = Map::find(
      dungeon,
      &state.progress.lock().unwrap().position.as_str(),
      0
    );
    let directions = Self::available_directions(terrain);

    if directions.left {
      let next_terrain = &mut terrain.children[0];
      let id = next_terrain.id.clone();

      Self::change_position(state, next_terrain, id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_right(state: &Engine) {
    println!("{}", t!("move.success"));
    let dungeon = &mut state.map.lock().unwrap().dungeon;
    let terrain = Map::find(
      dungeon,
      &state.progress.lock().unwrap().position.as_str(),
      0
    );
    let directions = Self::available_directions(terrain);

    if directions.right {
      let length = &terrain.children.len();
      let next_terrain = &mut terrain.children[length - 1];
      let id = next_terrain.id.clone();

      Self::change_position(state, next_terrain, id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_back(state: &Engine) {
    println!("{}", t!("move.success_back"));
    let dungeon = &mut state.map.lock().unwrap().dungeon;
    let terrain = Map::find(
      dungeon,
      &state.progress.lock().unwrap().position.as_str(),
      0
    );
    let directions = Self::available_directions(terrain);

    if directions.back {
      let id = terrain.id.substring(0, terrain.id.len() - 1);

      state.progress.lock().unwrap().position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_to_dungeon_entrance(state: &Engine) {
    println!("{}", t!("move.return_to_dungeon_entrance"));
    state.progress.lock().unwrap().position = String::from("0");
  }

  pub fn explore(state: &Engine) {
    let dungeon = &mut state.map.lock().unwrap().dungeon;

    let terrain = Map::find(
      dungeon,
      &state.progress.lock().unwrap().position,
      0,
    );

    let directions = Self::available_directions(terrain);

    println!("{}", t!("move.exploration.look_around"));

    if directions.left {
      println!("{}", t!("move.exploration.left_available"));
    }

    if directions.forward {
      println!("{}", t!("move.exploration.forward_available"));
    }

    if directions.right {
      println!("{}", t!("move.exploration.right_available"));
    }

    if directions.back {
      println!("{}", t!("move.exploration.back_available"));
      println!("{}", t!("move.exploration.entrance_available"));
    }
  }

  fn change_position(state: &Engine, next_terrain: &mut Terrain, id: String) {
    state.progress.lock().unwrap().position = String::from(id);

    if next_terrain.visited {
      println!("{}", t!("move.been_there"));
    } else {
      next_terrain.visited = true;
      Self::unravel_content(&next_terrain.content, state);
    }
  }

  fn available_directions(terrain: &Terrain) -> Directions {
    let n_of_routes = terrain.children.len();

    let mut directions = Directions::new();

    match n_of_routes {
      1 => {
        directions.forward = true;
        directions.forward_route_index = 0;
      },
      2 => {
        directions.left = true;
        directions.right = true;
      },
      3 => {
        directions.forward = true;
        directions.left = true;
        directions.right = true;
        directions.forward_route_index = 1;
      },
      _ => {}
    }

    if terrain.id == "0" {
      directions.back = false;
    }

    directions
  }

  fn unravel_content(content: &Content, state: &Engine) {
    match content {
      Content::Monster(monster)=> {
        state.progress.lock().unwrap().battle_mode = true;
        println!("{}", t!("content.monster.encounter"));
        println!("{}", t!("content.monster.behold", name = monster.name, health = monster.health, attack = monster.attack));

        if class_as_string(&monster.hates) == class_as_string(&state.player.lock().unwrap().class) {
          println!("{}", t!("content.monster.is_hater"));
        }

        if let MonsterLevel::Boss = monster.level {
          println!("{}", t!("content.monster.boss_found"));
        }
      },
      Content::Trap(trap) => {
        println!("{}", t!("content.trap.encounter"));
        state.progress.lock().unwrap().increment_traps_survived();

        match trap.class {
          TrapClass::StealAttack => {
            state.player.lock().unwrap().damage_weapon(1);
            println!("{}", t!("content.trap.damage_attack", damage = trap.damage));
            println!("{}", t!("player.remaining_attack", attack = state.player.lock().unwrap().attack));
          },
          TrapClass::StealDefence => {
            state.player.lock().unwrap().damage_armor(1);
            println!("{}", t!("content.trap.damage_defence", damage = trap.damage));
            println!("{}", t!("player.remaining_defence", defence = state.player.lock().unwrap().defence));
          },
          TrapClass::StealLife => {
            state.player.lock().unwrap().damage_health(trap.damage);
            println!("{}", t!("content.trap.damage_health", damage = trap.damage));
            println!("{}", t!("player.remaining_health", health = state.player.lock().unwrap().health));
          }
        }
      },
      Content::Treasure(item) => {
        println!("{}", t!("content.treasure.encounter"));
        state.player.lock().unwrap().equip(item);
        state.progress.lock().unwrap().increment_items_found();
      }
      Content::Empty => {
        println!("{}", t!("content.empty"));
      }
    }
  }
}