#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{SystemData, Resources, Entities, ReadStorage, WriteStorage, Write, ReadExpect, Read, Join},
    input::{InputHandler, InputEvent,},
    shrev::{EventChannel, ReaderId},
};
use crate::game_data::SurvivalState;
use crate::settings::Context;
use crate::components;
use crate::actions;
use crate::actions::{Action, Direction};

#[derive(Default)]
pub struct System {
    input_reader: Option<ReaderId<InputEvent<String>>>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, SurvivalState>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, EventChannel<InputEvent<String>>>,
        Entities<'s>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Actionable>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.input_reader = Some(Write::<EventChannel<InputEvent<String>>>::fetch(&res).register_reader());
    }

    #[allow(clippy::cast_possible_truncation)]
    fn run(&mut self, (_, mut state, input, input_events, entities, players, mut actionables): Self::SystemData) {
        if *state == SurvivalState::Paused {
            for (_, _, actionable) in (&entities, &players, &mut actionables).join() {

                let mut got_input = false;

                // hold-down key actions go here
                if input.action_is_down("move_up").unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_up");
                    actionable.channel.single_write(Action::Move(Direction::N));
                    got_input = true;
                }
                if input.action_is_down("move_down").unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_down");
                    actionable.channel.single_write(Action::Move(Direction::S));
                    got_input = true;
                }
                if input.action_is_down("move_left").unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_left");
                    actionable.channel.single_write(Action::Move(Direction::W));
                    got_input = true;
                }
                if input.action_is_down("pickup").unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_right");
                    actionable.channel.single_write(Action::TryPickup(actions::PickupTarget::Under));
                    got_input = true;
                }

                // Single shot event actions go here
                if !got_input {
                    for event in input_events.read(self.input_reader.as_mut().unwrap()) {
                        if let InputEvent::ActionPressed(action) = event {
                            match action.as_str() {
                                _ => {},
                            }
                        }
                    }
                }

                // End state
                if got_input {
                    *state = SurvivalState::Running;
                }
            }
        }
    }
}