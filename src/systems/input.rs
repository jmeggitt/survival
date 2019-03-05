#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{world, Entity, Entities, ReadStorage, WriteStorage, WriteExpect, Write, ReadExpect, Read, Join},
    input::InputHandler,
    shrev::EventChannel,
};
use crate::settings::Context;
use crate::components::{Player, Actionable, PawnAction};
use crate::systems::time::TimeState;


#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        WriteExpect<'s, TimeState>,
        Entities<'s>,
        WriteStorage<'s, Actionable>,
        ReadStorage<'s, Player>,

        Write<'s, EventChannel<(Entity, PawnAction)>>,
        Read<'s, InputHandler<String, String>>,
    );

    #[allow(clippy::cast_possible_truncation)]
    fn run(&mut self, (_, mut time_state, entities, mut actionables, players, mut action_channel, input): Self::SystemData) {
        for (entity, actionable, _) in (&entities, &mut actionables, &players).join() {
            let mut time_taken: u64 = 0;

            let x_move = input.axis_value("entity_x").unwrap() as f32 * 20.0;
            let y_move = input.axis_value("entity_y").unwrap() as f32 * 20.0;

            if x_move != 0. || y_move != 0. {
                action_channel.single_write((entity, PawnAction::Move(x_move, y_move)));
                time_taken = actionable.move_speed;
            }

            if time_taken > 0 {
                time_state.increment(time_taken);
            }
            // Increment the time state to allow for movement

        }
    }
}
