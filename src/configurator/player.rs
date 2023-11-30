use rand::Rng;
use std::io;
use rust_i18n::t;
use crate::data::enumerables::{
  Player, Class, Weapon, Armor, WeaponClass, ArmorClass,
};

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
    description: String::new(),
  }
}

fn build_default_staff() -> Weapon {
  Weapon {
    class: WeaponClass::Staff,
    belongs_to: Class::Mage,
    name: String::new(),
    attack: 3,
    description: String::new(),
  }
}

fn build_default_dagger() -> Weapon {
  Weapon {
    class: WeaponClass::Dagger,
    belongs_to: Class::Rogue,
    name: String::new(),
    attack: 2,
    description: String::new(),
  }
}

fn build_default_shield() -> Armor {
  Armor {
    class: ArmorClass::Shield,
    belongs_to: Class::Warrior,
    name: String::new(),
    defence: 5,
    description: String::new(),
  }
}

fn build_default_sphere() -> Armor {
  Armor {
    class: ArmorClass::Sphere,
    belongs_to: Class::Mage,
    name: String::new(),
    defence: 4,
    description: String::new(),
  }
}

fn build_default_cloak() -> Armor {
  Armor {
    class: ArmorClass::Cloak,
    belongs_to: Class::Rogue,
    name: String::new(),
    defence: 3,
    description: String::new(),
  }
}

fn build_default_warrior() -> Player {
  println!("{} {}", t!("player.created"), t!("player.class.warrior"));

  Player {
    class: Class::Warrior,
    health: 25,
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
    health: 20,
    attack: 8,
    defence: 5,
    weapon: build_default_staff(),
    armor: build_default_sphere(),
  }
}

fn build_default_rogue() -> Player {
  println!("{} {}", t!("player.created"), t!("player.class.rogue"));

  Player {
    class: Class::Rogue,
    health: 30,
    attack: 10,
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