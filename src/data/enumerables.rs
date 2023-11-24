pub enum Class {
  Warrior,
  Mage,
  Rogue,
  Any,
}

pub struct Hero {
  pub class: Class,
  pub health: i16,
  pub attack: i16,
  pub weapon: Treasure,
  pub armor: Treasure,
}

pub enum WeaponClass {
  Sword,
  Staff,
  Dagger,
}

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
    damage: i16,
    description: String,
  },
  Treasure {
    content: Treasure,
    description: String,
  },
  Empty,
}