#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{ Read, Entity, Resources, SystemData, ReadExpect, WriteStorage, ReadStorage, DenseVecStorage, },
    shrev::{EventChannel, ReaderId},
    core::components::Transform,
};
use specs_derive::Component;
use crate::settings::Context;
use crate::systems::time::TimeState;
use crate::components::{Player};

#[derive(Default)]
pub struct System {
    components: hibitset::BitSet,
    new_components: hibitset::BitSet,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

    }

    fn run(&mut self, _: Self::SystemData) {

    }
}