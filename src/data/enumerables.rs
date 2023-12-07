use rust_i18n::t;

pub enum Class {
  Warrior,
  Mage,
  Rogue,
  Any,
}

pub struct Player {
  pub class: Class,
  pub health: i8,
  pub attack: i8,
  pub defence: i8,
  pub weapon: Weapon,
  pub armor: Armor,
}

impl Player {
  pub fn equip(&mut self, item: Item) {
    match item {
      Item::Weapon(weapon) => {
        let current_weapon = &self.weapon;
        let weapon_class = Player::get_class_as_str(&weapon.belongs_to);
        let hero_class = Player::get_class_as_str(&self.class);

        if weapon_class != hero_class {
          println!("{}", t!("items.unusable"));
          return;
        }

        if weapon.attack > current_weapon.attack {
          self.attack = self.attack - current_weapon.attack + weapon.attack;
          println!("{}", t!("items.equipped_successfully"));
        } else {
          println!("{}", t!("items.too_weak"));
        }
      },
      Item::Armor(armor) => {
        let current_armor = &self.armor;
        let armor_class = Player::get_class_as_str(&armor.belongs_to);
        let hero_class = Player::get_class_as_str(&self.class);

        if armor_class != hero_class {
          println!("{}", t!("items.unusable"));
          return;
        }

        if armor.defence > current_armor.defence {
          self.defence = self.defence - current_armor.defence + armor.defence;
          println!("{}", t!("items.equipped_successfully"));
        } else {
          println!("{}", t!("items.too_weak"));
        }
      },
      Item::HealthPotion(health_potion) => {
        self.health += health_potion.power;
      },
      Item::Empty => {}
    }
  }

  fn get_class_as_str(val: &Class) -> String {
    match val {
      Class::Warrior => "warrior".to_string(),
      Class::Mage => "mage".to_string(),
      Class::Rogue => "rogue".to_string(),
      _ => "unknown".to_string(),
    }
  }
}

pub enum Item {
  Weapon(Weapon),
  Armor(Armor),
  HealthPotion(HealthPotion),
  Empty,
}

pub struct Weapon {
  pub class: WeaponClass,
  pub belongs_to: Class,
  pub name: String,
  pub attack: i8,
  pub description: String,
}

pub struct Armor {
  pub class: ArmorClass,
  pub belongs_to: Class,
  pub name: String,
  pub defence: i8,
  pub description: String,
}

pub struct HealthPotion {
  pub power: i8,
  pub description: String
}

#[derive(Clone)]
pub enum WeaponClass {
  Sword,
  Staff,
  Dagger,
}

#[derive(Clone)]
pub enum ArmorClass {
  Shield,
  Sphere,
  Cloak,
}

pub enum Treasure {
  Weapon {
    class: WeaponClass,
    name: String,
    attack: i8,
    description: String
  },
  Armor {
    class: ArmorClass,
    name: String,
    defence: i8,
    description: String
  },
  HealthPotion {
    power: i8,
    description: String
  },
  Empty
}

pub enum TrapClass {
  StealLife,
  StealAttack,
  StealDefence,
}

pub enum MonsterLevel {
  Weak,
  Average,
  Strong,
  Boss
}

pub enum ContentType {
  Monster,
  Trap,
  Treasure,
  Empty,
}

pub enum Content {
  Monster {
    name: String,
    health: i16,
    attack: i16,
    level: MonsterLevel,
    hates: Class,
    description: String,
    loot: Box<Content>,
  },
  Trap {
    class: TrapClass,
    damage: i8,
    description: String,
  },
  Treasure {
    content: Treasure,
    description: String,
  },
  Empty,
}