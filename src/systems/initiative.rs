#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{ Read, Entity, Resources, SystemData, ReadExpect, WriteStorage, ReadStorage, DenseVecStorage, },
    shrev::{EventChannel, ReaderId},
    core::components::Transform,
};
use specs_derive::Component;
use crate::settings::{Context};
use crate::game_data::SurvivalState;
use crate::systems::time::TimeState;
use crate::components;

#[derive(Default)]
pub struct System {

}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, SurvivalState>,
        WriteStorage<'s, components::IsTurn>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

    }

    fn run(&mut self, (_, state, is_turns): Self::SystemData) {
        match state {
            SurvivalState::Paused => {
                // do nothing?
            },
            SurvivalState::Running => {
                // Handle monster initiative, and handing it back to the player.
                *state == SurvivalState::Paused;
            },
        }
    }
}