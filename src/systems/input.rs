#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{world, Entity, Entities, ReadStorage, WriteStorage, WriteExpect, Write, ReadExpect, Read, Join},
    input::InputHandler,
    shrev::EventChannel,
};
use crate::game_data::SurvivalState;
use crate::settings::Context;
use crate::components;

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, SurvivalState>,
        Read<'s, InputHandler<String, String>>,
        Entities<'s>,
        ReadStorage<'s, components::Player>,
    );

    #[allow(clippy::cast_possible_truncation)]
    fn run(&mut self, (_, mut state, input, entities, players,): Self::SystemData) {
        if *state == SurvivalState::Paused {
            for (entity, _, ) in (&entities, &players, ).join() {
                let mut got_input = false;

                if input.action_is_down("move_up").unwrap() {

                    got_input = true;
                }

                // End state
                if got_input {
                    *state = SurvivalState::Running;
                }
            }
        }
    }
}