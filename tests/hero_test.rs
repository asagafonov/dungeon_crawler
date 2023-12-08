use dungeon_crawler::data::enumerables::{Player, Class, Weapon, WeaponClass, Armor, ArmorClass, Item};

#[test]
fn stronger_weapon_is_equipped() {
  let mut warrior = create_warrior();
  let stronger_sword = Weapon {
    class: WeaponClass::Sword,
    belongs_to: Class::Warrior,
    name: String::from("Stronger sword"),
    attack: 10,
    description: String::from("A much stronger sword"),
  };

  let expected_warrior_attack = warrior.attack - warrior.weapon.attack + stronger_sword.attack;

  warrior.equip(&Item::Weapon(stronger_sword));

  assert_eq!(warrior.attack, expected_warrior_attack);
}

#[test]
fn weaker_weapon_is_ignored() {
  let mut warrior = create_warrior();

  let weaker_sword = Weapon {
    class: WeaponClass::Sword,
    belongs_to: Class::Warrior,
    name: String::from("Weaker sword"),
    attack: 2,
    description: String::from("A much weaker sword"),
  };

  let expected_warrior_attack = warrior.attack;
  warrior.equip(&Item::Weapon(weaker_sword));

  assert_eq!(warrior.attack, expected_warrior_attack);
}

#[test]
fn another_class_weapon_is_ignored() {
  let mut warrior = create_warrior();
  let old_warrior_attack = warrior.attack;

  let staff = Weapon {
    class: WeaponClass::Staff,
    belongs_to: Class::Mage,
    name: String::from("A staff"),
    attack: 10,
    description: String::from("Shoots fireballs"),
  };

  warrior.equip(&Item::Weapon(staff));
  assert_eq!(warrior.attack, old_warrior_attack);
}

// helpers

fn create_warrior() -> Player {
  Player {
   class: Class::Warrior,
   health: 20,
   attack: 5,
   defence: 5,
   weapon: Weapon { class: WeaponClass::Sword, belongs_to: Class::Warrior, name: String::from("sword"), attack: 5, description: String::from("It cuts") },
   armor: Armor { class: ArmorClass::Shield, belongs_to: Class::Warrior , name: String::from("shield"), defence: 5, description: String::from("A little rusty one")},
 }
}