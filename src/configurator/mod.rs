use crate::data::enumerables::Hero;
use self::{map::Map, player::create_player};

pub mod player;
pub mod map;
pub mod locale;

pub fn configure_state() -> (Map, Hero) {
  let hero = create_player();
  let mut map = Map::build();
  map.insert_boss();

  (map, hero)
}