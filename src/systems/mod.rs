use amethyst::{
    ecs::{ System, Join, WriteStorage, ReadStorage, Read, ReadExpect},
    core::components::Transform,
    input::InputHandler,
};

use crate::components::Player;
use crate::settings::Context;

pub mod tile_position;
pub use tile_position::System as TilePositionSystem;

pub mod time;
pub use time::System as TimeSystem;

pub mod script;
pub use script::System as ScriptSystem;

#[derive(Default)]
pub struct ActionSystem;
impl<'s> System<'s> for ActionSystem {
    type SystemData = (
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, _: Self::SystemData) {

    }
}

#[derive(Default)]
pub struct PlayerInputSystem;
impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        ReadExpect<'s, Context>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    #[allow(clippy::cast_possible_truncation)]
    fn run(&mut self, (_, mut transforms, players, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.translate_x(x_move as f32 * 20.0);
            transform.translate_y(y_move as f32 * 20.0);
        }
    }
}