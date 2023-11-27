use rust_i18n::t;
use std::io;

pub fn set_locale() {
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