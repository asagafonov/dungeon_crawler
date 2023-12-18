use rand::Rng;
use std::io;
use rust_i18n::t;
use crate::data::types::*;
use crate::shared::helpers::*;

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
        let current_weapon = self.weapon.clone();
        let weapon_class = class_as_string(&weapon.belongs_to);
        let hero_class = class_as_string(&self.class);

        println!(
          "{}",
          t!(
            "items.define",
            item = weapon_as_string(&weapon.class),
            name = weapon.name,
            power = weapon.attack,
          ),
        );

        if weapon_class != hero_class {
          println!("{}", t!("items.unusable"));
          return;
        }

        if weapon.attack > current_weapon.attack {
          self.weapon = weapon.clone();
          self.attack = self.attack - current_weapon.attack + weapon.attack;
          println!("{}", t!("items.equipped_successfully"));
        } else {
          println!("{}", t!("items.too_weak"));
        }
      },
      Item::Armor(armor) => {
        let current_armor = &self.armor.clone();
        let armor_class = class_as_string(&armor.belongs_to);
        let hero_class = class_as_string(&self.class);

        println!(
          "{}",
          t!(
            "items.define",
            item = armor_as_string(&armor.class),
            name = armor.name,
            power = armor.defence,
          ),
        );

        if armor_class != hero_class {
          println!("{}", t!("items.unusable"));
          return;
        }

        if armor.defence > current_armor.defence {
          self.armor = armor.clone();
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

  pub fn damage_weapon(&mut self, damage: i8) {
    let new_attack = self.weapon.attack - damage;
    if new_attack <= 0 {
      self.weapon.attack = 0;
    } else {
      self.weapon.attack = new_attack;
      self.attack -= damage;
    }
  }

  pub fn damage_armor(&mut self, damage: i8) {
    let new_defence = self.armor.defence - damage;
    if new_defence <= 0 {
      self.armor.defence = 0;
    } else {
      self.armor.defence = new_defence;
      self.defence -= damage;
    }
  }

  pub fn damage_health(&mut self, damage: i8) {
    self.health -= damage;
  }
}

pub fn create_player() -> Player {
  println!("{}", t!("player.create"));
  println!(
    "[1] -> {}, [2] -> {}, [3] -> {}, [4] -> {}",
    t!("player.class.warrior"),
    t!("player.class.mage"),
    t!("player.class.rogue"),
    t!("player.class.random"),
  );

  let mut user_input = String::new();

  io::stdin().read_line(&mut user_input).expect("Failed to read line");

  let choice: i8 = match user_input.trim().parse() {
    Ok(num) => num,
    Err(_) => -1
  };

  match choice {
    1 => build_default_warrior(),
    2 => build_default_mage(),
    3 => build_default_rogue(),
    _ => build_random_class(),
  }
}

fn build_default_sword() -> Weapon {
  Weapon {
    class: WeaponClass::Sword,
    belongs_to: Class::Warrior,
    name: String::new(),
    attack: 5,
  }
}

fn build_default_staff() -> Weapon {
  Weapon {
    class: WeaponClass::Staff,
    belongs_to: Class::Mage,
    name: String::new(),
    attack: 3,
  }
}

fn build_default_dagger() -> Weapon {
  Weapon {
    class: WeaponClass::Dagger,
    belongs_to: Class::Rogue,
    name: String::new(),
    attack: 2,
  }
}

fn build_default_shield() -> Armor {
  Armor {
    class: ArmorClass::Shield,
    belongs_to: Class::Warrior,
    name: String::new(),
    defence: 5,
  }
}

fn build_default_sphere() -> Armor {
  Armor {
    class: ArmorClass::Sphere,
    belongs_to: Class::Mage,
    name: String::new(),
    defence: 4,
  }
}

fn build_default_cloak() -> Armor {
  Armor {
    class: ArmorClass::Cloak,
    belongs_to: Class::Rogue,
    name: String::new(),
    defence: 3,
  }
}

fn build_default_warrior() -> Player {
  println!("{} {}", t!("player.created"), t!("player.class.warrior"));

  Player {
    class: Class::Warrior,
    health: 30,
    attack: 5,
    defence: 10,
    weapon: build_default_sword(),
    armor: build_default_shield(),
  }
}

fn build_default_mage() -> Player {
   println!("{} {}", t!("player.created"), t!("player.class.mage"));

   Player {
    class: Class::Mage,
    health: 25,
    attack: 7,
    defence: 5,
    weapon: build_default_staff(),
    armor: build_default_sphere(),
  }
}

fn build_default_rogue() -> Player {
  println!("{} {}", t!("player.created"), t!("player.class.rogue"));

  Player {
    class: Class::Rogue,
    health: 25,
    attack: 9,
    defence: 3,
    weapon: build_default_dagger(),
    armor: build_default_cloak(),
  }
}

fn build_random_class() -> Player {
  let num: i8 = rand::thread_rng().gen_range(0..3);

  match num {
    0 => build_default_warrior(),
    1 => build_default_mage(),
    2 => build_default_rogue(),
    _ => panic!("no such class"),
  }
}