#[macro_use]
extern crate rust_i18n;

pub mod configurator;
pub mod data;

use std::io;
use rust_i18n::t;
use crate::configurator::{
    player::create_player,
    map::Map
};

i18n!("locales", fallback = "en");

pub fn start_game() {
    set_locale();
    let mut _hero = create_player();
    let mut map = Map::build();
    map.insert_boss();
}

fn set_locale() {
    let available_locales = rust_i18n::available_locales!();
    println!("{}: [{}]", "🌍 Choose your language", available_locales.join(", "));

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect(t!("errors.user_input").as_str());

    let user_input = user_input.trim();

    match available_locales.contains(&user_input) {
        true => {
            rust_i18n::set_locale(&user_input);
            println!("{} \"{}\"", t!("locales.set"), user_input);
        },
        false => {
            println!("{} \"en\"", t!("locales.set_default"));
        },
    }
}
