pub mod configurator;
pub mod data;
pub mod interactor;
pub mod engine;
mod shared;

use crate::configurator::{
    configure_state,
    locale::set_locale,
};
use crate::engine::Engine;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub fn start_game() {
    println!();
    println!("Welcome to DUNGEON CRAWLER!");
    println!();

    set_locale();

    let (map, player, progress) = configure_state();
    let engine = Engine::new(map, player, progress);

    engine.run();
}