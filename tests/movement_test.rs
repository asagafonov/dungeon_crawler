use dungeon_crawler::configurator::{
  map::{
    Terrain,
    Map,
  },
  progress::Progress,
};
use dungeon_crawler::data::enumerables::{Content, Player, Class, Weapon, Armor, WeaponClass, ArmorClass};
use dungeon_crawler::engine::Engine;
use dungeon_crawler::interactor::movement::MovementController;

#[test]
fn can_move_around() {
  let mut state = create_state();

  // can go left
  MovementController::go_left(&mut state);
  assert_eq!(String::from(&state.progress.position), String::from("00"));

  // can go back
  MovementController::go_backwards(&mut state);
  assert_eq!(String::from(&state.progress.position), String::from("0"));

  // can go right
  MovementController::go_right(&mut state);
  assert_eq!(String::from(&state.progress.position), String::from("01"));

  // can go back again
  MovementController::go_backwards(&mut state);
  assert_eq!(String::from(&state.progress.position), String::from("0"));

  // going forward doesn't change position, as forward direction doesn't exist
  MovementController::go_forward(&mut state);
  assert_eq!(String::from(&state.progress.position), String::from("0"));
}

// helpers

fn create_state() -> Engine {
  let map = Map {
    dungeon: Terrain {
    id: String::from("0"),
    content: Content::Empty,
    children: vec![
      Terrain {
        id: String::from("00"),
        content: Content::Empty,
        children: vec![],
        visited: false,
      },
      Terrain {
        id: String::from("01"),
        content: Content::Empty,
        children: vec![],
        visited: false,
      }
    ],
    visited: true,
    }
  };

  let progress = Progress::default();
  let player = Player {
    class: Class::Warrior,
    health: 20,
    attack: 8,
    defence: 4,
    weapon: Weapon {
      class: WeaponClass::Sword,
      belongs_to: Class::Warrior,
      name: String::new(),
      attack: 5,
      description: String::new(),
    },
    armor: Armor {
      class: ArmorClass::Shield,
      belongs_to: Class::Warrior,
      name: String::new(),
      defence: 5,
      description: String::new(),
    },
  };

  Engine::new(map, player, progress)
}