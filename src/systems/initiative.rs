use amethyst::ecs::{ReadExpect, Resources, SystemData, Write, WriteStorage};

use crate::components;
use crate::game_data::SurvivalState;
use crate::settings::Context;

#[derive(Default)]
pub struct System;

impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, SurvivalState>,
        WriteStorage<'s, components::IsTurn>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (_, mut state, _): Self::SystemData) {
        match state.clone() {
            SurvivalState::Paused => {
                // do nothing?
            }
            SurvivalState::Running => {
                // Handle monster initiative, and handing it back to the player.
                *state = SurvivalState::Paused;
                //trace!(context.logs.root, "AI turn finished, pausing");
            }
            _ => {}
        }
    }
}
