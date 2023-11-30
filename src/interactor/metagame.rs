use crate::engine::Engine;

pub(crate) struct MetagameController;

impl MetagameController {
  pub(crate) fn execute(input: &str, state: &Engine) {
    println!("Message from meta: {}", input);
  }
}