use rust_i18n::t;

#[derive(Clone)]
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
  pub fn equip(&mut self, item: &Item) {
    match item {
      Item::Weapon(weapon) => {
        let current_weapon = &self.weapon;
        let weapon_class = Player::get_class_as_string(&weapon.belongs_to);
        let hero_class = Player::get_class_as_string(&self.class);
        println!("{}", t!("items.define", item = Player::get_weapon_as_string(&current_weapon.class), power = current_weapon.attack));

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
        let armor_class = Player::get_class_as_string(&armor.belongs_to);
        let hero_class = Player::get_class_as_string(&self.class);
        println!("{}", t!("items.define", item = Player::get_armor_as_string(&current_armor.class), power = current_armor.defence));

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
        println!("{}", t!("items.drink_potion", power = health_potion.power));
        self.health += health_potion.power;
        println!("{}", t!("player.health_left", health = self.health));
      },
      Item::Empty => {}
    }
  }

  fn get_class_as_string(val: &Class) -> String {
    match val {
      Class::Warrior => "warrior".to_string(),
      Class::Mage => "mage".to_string(),
      Class::Rogue => "rogue".to_string(),
      _ => "unknown".to_string(),
    }
  }

  fn get_weapon_as_string(val: &WeaponClass) -> String {
    match val {
      WeaponClass::Sword => "sword".to_string(),
      WeaponClass::Staff => "staff".to_string(),
      WeaponClass::Dagger => "dagger".to_string(),
    }
  }

  fn get_armor_as_string(val: &ArmorClass) -> String {
    match val {
      ArmorClass::Shield => "shield".to_string(),
      ArmorClass::Sphere => "sphere".to_string(),
      ArmorClass::Cloak => "cloak".to_string(),
    }
  }
}

#[derive(Clone)]
pub enum Item {
  Weapon(Weapon),
  Armor(Armor),
  HealthPotion(HealthPotion),
  Empty,
}

#[derive(Clone)]
pub struct Weapon {
  pub class: WeaponClass,
  pub belongs_to: Class,
  pub name: String,
  pub attack: i8,
  pub description: String,
}

#[derive(Clone)]
pub struct Armor {
  pub class: ArmorClass,
  pub belongs_to: Class,
  pub name: String,
  pub defence: i8,
  pub description: String,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub enum TrapClass {
  StealLife,
  StealAttack,
  StealDefence,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Monster {
  pub name: String,
  pub health: i8,
  pub attack: i8,
  pub level: MonsterLevel,
  pub hates: Class,
  pub loot: Box<Content>,
}

#[derive(Clone)]
pub struct Trap {
  pub class: TrapClass,
  pub damage: i8,
}

#[derive(Clone)]
pub enum Content {
  Monster(Monster),
  Trap(Trap),
  Treasure(Item),
  Empty,
}