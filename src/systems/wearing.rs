#![allow(clippy::module_name_repetitions)]

use crate::components;
use crate::settings::Context;
use amethyst::{
    core::{Parent, ParentHierarchy},
    ecs::{Entities, ReadExpect, WriteStorage},
};

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, ParentHierarchy>,
        Entities<'s>,
        WriteStorage<'s, components::Item>,
        WriteStorage<'s, components::Container>,
        WriteStorage<'s, Parent>,
    );

    fn run(&mut self, _: Self::SystemData) {
        //let (entity, _) = (&entities, &items).join().next().unwrap();
        //let container_bitset = inventory::get_all_containers(entity, &hierarchy, &containers);
    }
}
