use rust_i18n::t;
use substring::Substring;
use crate::{
  engine::Engine,
  configurator::map::{
    Map,
    Terrain,
  },
  data::enumerables::Content,
};

pub struct Directions {
  forward: bool,
  left: bool,
  right: bool,
  backwards: bool,
  forward_route_index: usize,
}

impl Directions {
  pub fn new() -> Directions {
    Directions {
      forward: false,
      left: false,
      right: false,
      backwards: true,
      forward_route_index: 0,
    }
  }
}

pub struct MovementController;

impl MovementController {
  pub fn go_forward(state: &Engine) {
    let dungeon = &state.map.lock().unwrap().dungeon;
    let (_, terrain) = Map::find_by(
      dungeon,
      &state.progress.lock().unwrap().position,
    );
    let directions = Self::available_directions(terrain);

    if directions.forward {
      println!("{}", t!("move.success"));
      let next_terrain = &terrain.children[directions.forward_route_index];
      Self::unravel_content(&next_terrain.content, state);
      let id = &next_terrain.id;

      state.progress.lock().unwrap().position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_left(state: &Engine) {
    println!("{}", t!("move.success"));
    let dungeon = &state.map.lock().unwrap().dungeon;
    let (_, terrain) = Map::find_by(
      dungeon,
      &state.progress.lock().unwrap().position,
    );
    let directions = Self::available_directions(terrain);

    if directions.left {
      let next_terrain = &terrain.children[0];
      Self::unravel_content(&next_terrain.content, state);
      let id = &next_terrain.id;

      state.progress.lock().unwrap().position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_right(state: &Engine) {
    println!("{}", t!("move.success"));
    let dungeon = &state.map.lock().unwrap().dungeon;
    let (_, terrain) = Map::find_by(
      dungeon,
      &state.progress.lock().unwrap().position,
    );
    let directions = Self::available_directions(terrain);

    if directions.right {
      let next_terrain = &terrain.children[&terrain.children.len() - 1];
      Self::unravel_content(&next_terrain.content, state);
      let id = &next_terrain.id;

      state.progress.lock().unwrap().position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_backwards(state: &Engine) {
    println!("{}", t!("move.success_backwards"));
    let dungeon = &state.map.lock().unwrap().dungeon;
    let (_, terrain) = Map::find_by(
      dungeon,
      &state.progress.lock().unwrap().position,
    );
    let directions = Self::available_directions(terrain);

    if directions.backwards {
      let id = terrain.id.substring(0, terrain.id.len() - 1);

      state.progress.lock().unwrap().position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn explore(state: &Engine) {
    let dungeon = &state.map.lock().unwrap().dungeon;

    let (_, terrain) = Map::find_by(
      &dungeon,
      &state.progress.lock().unwrap().position,
    );

    let directions = Self::available_directions(terrain);

    println!("{}", t!("move.exploration.look_around"));

    if directions.forward {
      println!("{}", t!("move.exploration.forward_available"));
    }

    if directions.left {
      println!("{}", t!("move.exploration.left_available"));
    }

    if directions.right {
      println!("{}", t!("move.exploration.right_available"));
    }

    if directions.backwards {
      println!("{}", t!("move.exploration.backwards_available"));
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
      directions.backwards = false;
    }

    directions
  }

  fn unravel_content(content: &Content, state: &Engine) {
    match content {
      Content::Monster {
        name,
        level,
        hates,
        ..
      } => {
        state.progress.lock().unwrap().battle_mode = true;
        println!("{}", t!("content.monster.encounter"));
        println!("{} {}", t!("content.monster.behold"), name);
      },
      Content::Trap { damage, .. } => {
        println!("{}", t!("content.trap.encounter"));

        state.player.lock().unwrap().health -= damage;
        println!("{} {} {}", t!("content.trap.deals"), damage, t!("content.trap.damage"));
        println!("{} {}", t!("player.remaining_health"), state.player.lock().unwrap().health);
      },
      Content::Treasure { .. } => {
        println!("{}", t!("content.treasure.encounter"));
      }
      Content::Empty => {
        println!("{}", t!("content.empty"));
      }
    }
  }
}