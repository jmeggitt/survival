#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{ Component, Read, Entity, Resources, SystemData, ReadExpect, WriteStorage, ReadStorage, DenseVecStorage, Entities, Join},
    shrev::{EventChannel, ReaderId},
    core::components::Transform,

};
use specs_derive::Component;
use crate::settings::Context;
use crate::systems::time::TimeState;
use crate::components::{Player};

use crate::components;
use crate::utils::ComponentEventReader;

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, crate::actions::Action, <components::Actionable as Component>::Storage>,

}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        WriteStorage<'s, components::Actionable>,
        WriteStorage<'s, Transform>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

       self.action_reader.setup(res);;
    }

    fn run(&mut self, (_, entities, mut actionables, _): Self::SystemData) {
        self.action_reader.maintain(&entities, &mut actionables);

        // Read components...
        for (entity, mut actionable) in (&entities, &mut actionables).join() {
            let events = self.action_reader.read(entity, &mut actionable);

        }
    }
}