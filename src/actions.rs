#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum Direction {
    N,
    NW,
    NE,
    S,
    SW,
    SE,
    E,
    W,
}
impl Default for Direction { fn default() -> Self { Direction::N } }

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum Action {
    Move(Direction),
    Wait,
}
impl Default for Action { fn default() -> Self { Action::Wait } }
