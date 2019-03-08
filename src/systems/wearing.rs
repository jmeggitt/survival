#![allow(clippy::module_name_repetitions)]

use amethyst::{
    core::{ParentHierarchy, Parent},
    ecs::{Join, ReadStorage, WriteStorage, Read, ReadExpect, Entity, Entities},
};
use crate::settings::Context;
use crate::components;
use hibitset::BitSet;
use crate::inventory;

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

    fn run(&mut self, (_, hierarchy,
                        entities, items, containers, parents)
    : Self::SystemData) {
        //let (entity, _) = (&entities, &items).join().next().unwrap();
        //let container_bitset = inventory::get_all_containers(entity, &hierarchy, &containers);
    }
}
