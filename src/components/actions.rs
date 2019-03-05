#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum PawnAction {
    Move(f32, f32),
    Wait,
}
impl Default for PawnAction {
    fn default() -> Self {
        PawnAction::Wait
    }
}