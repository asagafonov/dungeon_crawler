use crate::data::types::{Class, WeaponClass, ArmorClass};

pub fn class_as_string(class: &Class) -> String {
  match class {
    Class::Warrior => "warrior".to_string(),
    Class::Mage => "mage".to_string(),
    Class::Rogue => "rogue".to_string(),
    _ => "unknown".to_string(),
  }
}

pub fn weapon_as_string(val: &WeaponClass) -> String {
  match val {
    WeaponClass::Sword => "sword".to_string(),
    WeaponClass::Staff => "staff".to_string(),
    WeaponClass::Dagger => "dagger".to_string(),
  }
}

pub fn armor_as_string(val: &ArmorClass) -> String {
  match val {
    ArmorClass::Shield => "shield".to_string(),
    ArmorClass::Sphere => "sphere".to_string(),
    ArmorClass::Cloak => "cloak".to_string(),
  }
}