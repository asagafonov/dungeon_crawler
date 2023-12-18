use rust_i18n::t;

pub struct Progress {
  pub is_boss_defeated: bool,
  pub battle_mode: bool,
  pub need_evac: bool,
  pub position: String,
  pub monsters_killed: i8,
  pub traps_survived: i8,
  pub items_found: i8,
}

impl Progress {
  pub fn default() -> Progress {
    Progress {
      is_boss_defeated: false,
      battle_mode: false,
      need_evac: false,
      position: String::from("0"),
      monsters_killed: 0,
      traps_survived: 0,
      items_found: 0,
    }
  }

  pub fn increment_monsters_killed(&mut self) {
    self.monsters_killed += 1;
  }

  pub fn increment_traps_survived(&mut self) {
    self.traps_survived += 1;
  }

  pub fn increment_items_found(&mut self) {
    self.items_found += 1;
  }

  pub fn show_statistics(&self) {
    println!();

    let score = (self.monsters_killed as i32 * 20) + (self.traps_survived as i32 * 5) + (self.items_found as i32 * 10);
    println!("{}", t!("game.result.score", score = score));
    println!("{}", t!("game.result.monsters_killed", amount = self.monsters_killed));
    println!("{}", t!("game.result.traps_survived", amount = self.traps_survived));
    println!("{}", t!("game.result.items_found", amount = self.items_found));

    let game_outcome = if self.is_boss_defeated { t!("game.result.victory") } else if self.need_evac { t!("game.result.runaway") } else { t!("game.result.death") };
    println!("{}", t!("game.result.outcome", outcome = game_outcome));
    println!();
  }
}