#[derive(Clone)]
pub enum Class {
  Warrior,
  Mage,
  Rogue,
  Any,
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
  pub loot: Item,
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