use rand::Rng;
use crate::data::{
  enumerables::{
    Content,
    Class,
    MonsterLevel,
    TrapClass,
    WeaponClass,
    ArmorClass,
    Monster,
    Trap,
    Item,
    Weapon,
    Armor,
    HealthPotion,
  },
  constants::MAX_DUNGEON_DEPTH,
};

pub struct Terrain {
  pub id: String,
  pub children: Vec<Terrain>,
  pub content: Content,
  pub visited: bool,
}

pub struct Map {
  pub dungeon: Terrain,
}

impl Map {
  pub fn build() -> Map {
    let starting_index = String::from("0");
    let dungeon: Terrain = Map::create_terrain(&starting_index);

    Map { dungeon }
  }

  pub fn find_by<'a>(dungeon: &'a Terrain, id: &String) -> (bool, &'a Terrain) {
    if id.eq(&dungeon.id) {
      return (true, dungeon);
    }

    for child in &dungeon.children {
      let (found_id, terrain) = Self::find_by(&child, id);

      if found_id {
        return (true, terrain)
      }
    }

    (false, dungeon)
  }

  fn create_terrain(id: &String) -> Terrain {
    if id.len() as i8 > MAX_DUNGEON_DEPTH {
      return Terrain {
        id: String::from(id),
        children: vec![],
        content: Map::generate_content(),
        visited: false,
      }
    }

    let number_of_children: i8 = rand::thread_rng().gen_range(1..=3);
    let mut children: Vec<Terrain> = vec![];

    for child_id in 0..number_of_children {
      children.push(Map::create_terrain(&format!("{}{}", id, child_id)));
    }

    Terrain {
      id: String::from(id),
      children,
      content: if id == "0" { Content::Empty} else { Map::generate_content() },
      visited: false,
    }
  }

  fn generate_content() -> Content {
    let content_index: i8 = rand::thread_rng().gen_range(0..=3);
    let power_level: i8 = rand::thread_rng().gen_range(0..=2);

    let treasure_level = match power_level {
      0 => MonsterLevel::Weak,
      1 => MonsterLevel::Average,
      _ => MonsterLevel::Strong
    };

    match content_index {
      0 => Map::generate_random_monster(),
      1 => Map::generate_random_treasure(treasure_level),
      2 => Map::generate_random_trap(),
      _ => Content::Empty,
    }
  }

  fn generate_random_monster() -> Content {
    let monster_index: i8 = rand::thread_rng().gen_range(0..=2);
    let opponent_index: i8 = rand::thread_rng().gen_range(0..=2);

    let opponent = match opponent_index {
      0 => Class::Warrior,
      1 => Class::Mage,
      _ => Class::Rogue,
    };

    match monster_index {
      1 => Content::Monster(Monster {
        name: String::new(),
        health: 10,
        attack: 3,
        level: MonsterLevel::Weak,
        hates: opponent,
        loot: Box::new(Map::generate_random_treasure(MonsterLevel::Weak)),
      }),
      2 => Content::Monster(Monster {
        name: String::new(),
        health: 15,
        attack: 5,
        level: MonsterLevel::Average,
        hates: opponent,
        loot: Box::new(Map::generate_random_treasure(MonsterLevel::Average)),
      }),
      _ => Content::Monster(Monster {
        name: String::new(),
        health: 20,
        attack: 7,
        level: MonsterLevel::Strong,
        hates: opponent,
        loot: Box::new(Map::generate_random_treasure(MonsterLevel::Strong)),
      }),
    }
  }

  fn generate_random_treasure(monster_level: MonsterLevel) -> Content {
    let treasure_class: i8 = rand::thread_rng().gen_range(0..=2);
    let character: i8 = rand::thread_rng().gen_range(0..=2);

    let (weapon_class, armor_class, hero_class) = match character {
      0 => (WeaponClass::Sword, ArmorClass::Shield, Class::Warrior),
      1 => (WeaponClass::Staff, ArmorClass::Sphere, Class::Mage),
      _ => (WeaponClass::Dagger, ArmorClass::Cloak, Class::Rogue),
    };

    let max_power: i8 = match monster_level {
      MonsterLevel::Weak => 4,
      MonsterLevel::Average => 6,
      MonsterLevel::Strong => 8,
      _ => 12,
    };

    let item_power = rand::thread_rng().gen_range(2..=max_power);

    match treasure_class {
      0 => { Content::Treasure(Item::Weapon(Weapon {
          class: weapon_class,
          name: String::new(),
          attack: item_power,
          description: String::new(),
          belongs_to: hero_class,
        }))
      },
      1 => { Content::Treasure(Item::Armor(Armor {
          class: armor_class,
          name: String::new(),
          defence: item_power,
          description: String::new(),
          belongs_to: hero_class,
        }))
      },
      _ => {
        Content::Treasure(Item::HealthPotion( HealthPotion{
          power: item_power,
          description: String::new(),
      }))
    },
  }
}

  fn generate_random_trap() -> Content {
    let trap_class: i8 = rand::thread_rng().gen_range(0..=2);

    match trap_class {
      0 => Content::Trap(Trap { class: TrapClass::StealAttack, damage: 1, }),
      1 => Content::Trap(Trap { class: TrapClass::StealDefence, damage: 1, }),
      _ => Content::Trap(Trap { class: TrapClass::StealLife, damage: 2 }),
    }
  }

  pub fn insert_boss(&mut self) {
    let boss = Content::Monster(Monster {
      name: String::new(),
      health: 30,
      attack: 10,
      level: MonsterLevel::Boss,
      hates: Class::Any,
      loot: Box::new(Map::generate_random_treasure(MonsterLevel::Boss)),
    });

    let mut index = "0";
    let mut current_terrain = &mut self.dungeon;

    while index.len() < (MAX_DUNGEON_DEPTH + 1) as usize {
      let len = current_terrain.children.len();
      let random_child = rand::thread_rng().gen_range(0..len);

      current_terrain = &mut current_terrain.children[random_child];
      index = current_terrain.id.as_str();
    }

    current_terrain.content = boss;
  }
}