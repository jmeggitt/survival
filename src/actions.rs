pub enum Action {
    Move,
    Wait,
}
impl Default for Action { fn default() -> Self { Action::Wait } }