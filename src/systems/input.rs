//#![allow(clippy::module_name_repetitions)]

use amethyst::{
    core::transform::Transform,
    ecs::{
        Entities, Join, Read, ReadExpect, ReadStorage, Resources, SystemData, Write, WriteStorage,
    },
    input::{InputEvent, InputHandler},
    renderer::Camera,
    shrev::{EventChannel, ReaderId},
};

use crate::actions;
use crate::actions::{Action, Direction, PlayerInputAction};
use crate::components;
use crate::game_data::SurvivalState;
use crate::settings::Context;

#[derive(Default)]
pub struct System {
    input_reader: Option<ReaderId<InputEvent<PlayerInputAction>>>,
}

impl<'s> amethyst::ecs::System<'s> for System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, SurvivalState>,
        Read<'s, InputHandler<PlayerInputAction, PlayerInputAction>>,
        Read<'s, EventChannel<InputEvent<PlayerInputAction>>>,
        Entities<'s>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Actionable>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.input_reader = Some(
            Write::<EventChannel<InputEvent<PlayerInputAction>>>::fetch(&res).register_reader(),
        );
    }

    #[allow(clippy::cast_possible_truncation)]
    fn run(
        &mut self,
        (
            _,
            mut state,
            input,
            input_events,
            entities,
            players,
            mut actionables,
            cameras,
            mut transforms, // for debuging
        ): Self::SystemData,
    ) {
        if *state == SurvivalState::Paused {
            for (_, _, actionable) in (&entities, &players, &mut actionables).join() {
                // TODO refactor to use match
                let mut got_input = false;

                // hold-down key actions go here
                if input.action_is_down(&PlayerInputAction::MoveUp).unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_up");
                    actionable.channel.single_write(Action::Move(Direction::N));
                    got_input = true;
                }
                if input.action_is_down(&PlayerInputAction::MoveDown).unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_down");
                    actionable.channel.single_write(Action::Move(Direction::S));
                    got_input = true;
                }
                if input.action_is_down(&PlayerInputAction::MoveLeft).unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_left");
                    actionable.channel.single_write(Action::Move(Direction::W));
                    got_input = true;
                }
                if input.action_is_down(&PlayerInputAction::MoveRight).unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_left");
                    actionable.channel.single_write(Action::Move(Direction::E));
                    got_input = true;
                }
                if input.action_is_down(&PlayerInputAction::PickUp).unwrap() {
                    //slog_trace!(context.logs.root, "Got player input in direction: move_right");
                    actionable
                        .channel
                        .single_write(Action::TryPickup(actions::PickupTarget::Under));
                    got_input = true;
                }

                if input.action_is_down(&PlayerInputAction::ZoomIn).unwrap() {
                    if let Some((_, transform)) = (&cameras, &mut transforms).join().next() {
                        *transform.scale_mut() = transform.scale() * 1.1;
                    }
                }
                if input.action_is_down(&PlayerInputAction::ZoomOut).unwrap() {
                    if let Some((_, transform)) = (&cameras, &mut transforms).join().next() {
                        *transform.scale_mut() = transform.scale() * 0.9;
                    }
                }

                // Single shot event actions go here
                if !got_input {
                    for event in input_events.read(self.input_reader.as_mut().unwrap()) {
                        if let InputEvent::ActionPressed(action) = event {
                            match action {
                                _ => {}
                            }
                        }
                    }
                }

                // End state
                if got_input {
                    *state = SurvivalState::Running;
                }

                // Set the camera position here too LOL
                let mut player_translation = None;
                if let Some((_, player_transform)) = (&players, &mut transforms).join().next() {
                    player_translation = Some(*player_transform.translation());
                }

                if let Some((_, transform)) = (&cameras, &mut transforms).join().next() {
                    if let Some(t) = player_translation {
                        *transform.translation_mut() = t;
                        // Offset the camera 200 right
                        //transform.move_right(200.);
                        transform.set_translation_z(1.0);
                    }
                }
            }
        }
    }
}
