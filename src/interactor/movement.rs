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
  pub fn go_forward(state: &mut Engine) {
    let (terrain, directions) = Self::available_directions(state);

    if directions.forward {
      println!("{}", t!("move.success"));
      let next_terrain = &terrain.children[directions.forward_route_index];
      Self::unravel_content(&next_terrain.content);
      let id = &next_terrain.id;

      state.progress.position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_left(state: &mut Engine) {
    println!("{}", t!("move.success"));
    let (terrain, directions) = Self::available_directions(state);

    if directions.left {
      let next_terrain = &terrain.children[0];
      Self::unravel_content(&next_terrain.content);
      let id = &next_terrain.id;

      state.progress.position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_right(state: &mut Engine) {
    println!("{}", t!("move.success"));
    let (terrain, directions) = Self::available_directions(state);

    if directions.right {
      let next_terrain = &terrain.children[&terrain.children.len() - 1];
      Self::unravel_content(&next_terrain.content);
      let id = &next_terrain.id;

      state.progress.position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn go_backwards(state: &mut Engine) {
    println!("{}", t!("move.success_backwards"));
    let (terrain, directions) = Self::available_directions(state);

    if directions.backwards {
      let id = terrain.id.substring(0, terrain.id.len() - 1);

      state.progress.position = String::from(id);
    } else {
      println!("{}", t!("move.direction_missing"));
      println!("{}", t!("move.command_ambiguous"));
    }
  }

  pub fn explore(state: &mut Engine) {
    let (_, directions) = Self::available_directions(state);

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

  fn available_directions(state: &mut Engine) -> (&Terrain, Directions) {
    let (_, current_terrain) = Map::find_by(&state.map.dungeon, &state.progress.position);
    let n_of_routes = current_terrain.children.len();

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

    if current_terrain.id == "0" {
      directions.backwards = false;
    }

    (current_terrain, directions)
  }

  fn unravel_content(content: &Content) {
    match content {
      Content::Monster { .. } => {
        println!("{}", t!("content.monster.encounter"));
      },
      Content::Trap { .. } => {
        println!("{}", t!("content.trap.encounter"));
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