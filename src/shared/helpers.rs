use crate::data::enumerables::Class;

pub fn class_as_string(class: &Class) -> String {
  match class {
    Class::Warrior => "warrior".to_string(),
    Class::Mage => "warrior".to_string(),
    Class::Rogue => "rogue".to_string(),
    _ => "".to_string(),
  }
}