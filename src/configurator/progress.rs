use crate::data::enumerables::Content;

pub struct Progress {
  pub is_boss_defeated: bool,
  pub battle_mode: bool,
  pub position: String,
  pub turns_taken: i8,
  pub monsters_killed: i8,
  pub traps_survived: i8,
  pub items_found: i8,
  pub score: i16,
  pub visited_ids: Vec<String>,
  pub monster: Content,
}

impl Progress {
  pub fn default() -> Progress {
    Progress {
      is_boss_defeated: false,
      battle_mode: false,
      position: String::from("0"),
      turns_taken: 0,
      monsters_killed: 0,
      traps_survived: 0,
      items_found: 0,
      score: 0,
      visited_ids: vec![],
      // monster: Monster {
      //   name: String::from(""),
      //   attack: 0,
      //   health: 0,
      //   level: MonsterLevel::Weak,
      //   hates: Class::Warrior,
      //   loot: Box::new(Treasure::HealthPotion {
      //     power: 0,
      //     description: String::from(""),
      //   }),
      // },
      monster: Content::Empty,
    }
  }
}