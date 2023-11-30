pub mod configurator;
pub mod data;
pub mod interactor;
mod engine;

use crate::configurator::{
    configure_state,
    locale::set_locale,
};
use crate::engine::Engine;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub fn start_game() {
    set_locale();

    let (map, player) = configure_state();
    let mut engine = Engine::new(map, player);

    engine.run();
}