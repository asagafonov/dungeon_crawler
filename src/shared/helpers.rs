use rust_i18n::t;
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
    WeaponClass::Sword => t!("items.sword"),
    WeaponClass::Staff => t!("items.staff"),
    WeaponClass::Dagger => t!("items.dagger"),
  }
}

pub fn armor_as_string(val: &ArmorClass) -> String {
  match val {
    ArmorClass::Shield => t!("items.shield"),
    ArmorClass::Sphere => t!("items.sphere"),
    ArmorClass::Cloak => t!("items.cloak"),
  }
}