use dungeon_crawler::{
  configurator::map::{Map, Terrain},
  data::{
    enumerables::{Content, MonsterLevel},
    constants::MAX_DUNGEON_DEPTH,
  }
};

#[test]
fn default_dungeon_has_no_boss() {
  let map = Map::build();
  let (found_boss, _) = search_for_boss(&map.dungeon);
  assert!(!found_boss);
}

#[test]
fn boss_is_reachable_at_dungeon_end() {
  let mut five_runs: Vec<bool> = vec![];

  for _ in 0..5 {
    let mut map = Map::build();
    map.insert_boss();
    let (found_boss, id) = search_for_boss(&map.dungeon);

    assert_eq!(id.len() - 1, MAX_DUNGEON_DEPTH as usize);
    five_runs.push(found_boss);
  }

  assert!(five_runs.iter().all(|res| *res == true));
}

// helpers

fn search_for_boss(dungeon: &Terrain) -> (bool, &str) {
  if matches!(dungeon.content, Content::Monster { level: MonsterLevel::Boss, .. }) {
    return (true, &dungeon.id);
  }

  for child in &dungeon.children {
    let (found_boss, id) = search_for_boss(&child);

    if found_boss {
      return (true, id)
    }
  }

  (false, "")
}