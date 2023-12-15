use rust_i18n::t;

pub struct Progress {
  pub is_boss_defeated: bool,
  pub battle_mode: bool,
  pub position: String,
  pub monsters_killed: i8,
  pub traps_survived: i8,
  pub items_found: i8,
  pub score: i16,
  pub need_evac: bool,
}

impl Progress {
  pub fn default() -> Progress {
    Progress {
      is_boss_defeated: false,
      battle_mode: false,
      position: String::from("0"),
      monsters_killed: 0,
      traps_survived: 0,
      items_found: 0,
      score: 0,
      need_evac: false,
    }
  }

  pub fn update_score(&mut self) {
    self.score = (self.monsters_killed * 20) as i16;
    self.score = (self.traps_survived * 5) as i16;
    self.score = (self.items_found * 10) as i16;
  }

  pub fn show_statistics(&self) {
    println!();
    println!("{}", t!("game.result.score", score = self.score));
    println!("{}", t!("game.result.monsters_killed", amount = self.monsters_killed));
    println!("{}", t!("game.result.traps_survived", amount = self.traps_survived));
    println!("{}", t!("game.result.items_found", amount = self.items_found));
    let game_outcome = if self.is_boss_defeated { t!("game.result.victory") } else if self.need_evac { t!("game.result.runaway") } else { t!("game.result.death") };
    println!("{}", t!("game.result.outcome", outcome = game_outcome));
    println!();
  }
}