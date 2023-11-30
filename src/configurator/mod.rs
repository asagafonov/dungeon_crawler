use crate::data::enumerables::Player;
use self::{map::Map, player::create_player};

pub mod player;
pub mod map;
pub mod locale;

pub fn configure_state() -> (Map, Player) {
  let player = create_player();
  let mut map = Map::build();
  map.insert_boss();

  (map, player)
}