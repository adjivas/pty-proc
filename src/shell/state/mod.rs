use super::command::Command;

pub type Key = u8;

pub struct State(
  pub Option<Command>,
  pub Option<Key>,
);

impl Default for State {
  fn default() -> State {
    State(None, None)
  }
}
