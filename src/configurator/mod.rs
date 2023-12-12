use crate::configurator::player::Player;
use self::{map::Map, player::create_player, progress::Progress};

pub mod player;
pub mod map;
pub mod locale;
pub mod progress;

pub fn configure_state() -> (Map, Player, Progress) {
  let player = create_player();
  let progress = Progress::default();
  let mut map = Map::build();
  map.insert_boss();

  (map, player, progress)
}