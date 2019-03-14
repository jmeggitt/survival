use amethyst::ecs::Entity;
use amethyst::core::math::Vector3;

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

#[derive(Clone, Copy)]
pub enum PickupTarget {
    Entity(Entity),
    Location(Vector3<f32>),
    Under,
}

#[derive(Clone, Copy)]
pub enum Action {
    Move(Direction),
    Drop(Entity),
    Wait,

    // Tryable Actions
    // One system handles the try action, and then broadcasts the Do Action which means its imminent
    TryPickup(PickupTarget),
    DoPickup(Entity),
}
impl Default for Action { fn default() -> Self { Action::Wait } }
