#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{ Read, Entity, Resources, SystemData, ReadExpect, WriteStorage, ReadStorage, },
    shrev::{EventChannel, ReaderId},
    core::components::Transform,
};
use crate::settings::Context;
use crate::systems::time::TimeState;
use crate::components::{Actionable, PawnAction, Player};

#[derive(Default)]
pub struct System {
    components: hibitset::BitSet,
    new_components: hibitset::BitSet,

    action_reader_id: Option<ReaderId<(Entity, PawnAction)>>,
    elapsed_event_reader_id: Option<ReaderId<u64>>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Read<'s, TimeState>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Actionable>,
        WriteStorage<'s, Transform>,
        Read<'s, EventChannel<(Entity, PawnAction)>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.elapsed_event_reader_id = Some(res.fetch_mut::<TimeState>().elapsed_event.register_reader());
        self.action_reader_id = Some(res.fetch_mut::<EventChannel<(Entity, PawnAction)>>().register_reader());
    }

    fn run(&mut self, (_, time, _, _actionables, mut transforms, action_channel): Self::SystemData) {
        for _time_elapsed in time.elapsed_event.read(self.elapsed_event_reader_id.as_mut().unwrap()) {
            for (entity, action) in action_channel.read(self.action_reader_id.as_mut().unwrap()) {
                // TODO: do we just assume we had the time avialable?
                // does the time system instead read the events too, and consume the appropriate times taken?
                if let Some(transform) = transforms.get_mut(*entity) {
                    match action {
                        PawnAction::Move(x, y) => {
                            transform.translate_x(*x);
                            transform.translate_y(*y);
                        },
                        _ => {},
                    };
                }
            }
        }
    }

        //for _time_elapsed in time.elapsed_event.read(self.elapsed_event_reader_id.as_mut().unwrap()) {
        /*
            for (actionable, transform, id) in (&mut actionables, &mut transforms).join() {
                for action in actionable.channel.read(self.action_reader_ids.get_mut(&id).unwrap()) {
                    slog_trace!(context.logs.root, "Got action!");
                    match action {
                        PawnAction::Move(x, y) => {
                            transform.translate_x(*x);
                            transform.translate_y(*y);
                        },
                        _ => {},
                    };
                }
            }
        //}*/

}