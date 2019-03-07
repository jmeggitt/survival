#![allow(clippy::module_name_repetitions)]

use amethyst::{
    core::{ParentHierarchy, Parent},
    ecs::{Join, ReadStorage, WriteStorage, Read, ReadExpect, Entity, Entities},
};
use crate::settings::Context;
use crate::components;
use hibitset::BitSet;

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
        let (entity, _) = (&entities, &items).join().next().unwrap();
        let container_bitset = get_containers(entity, &hierarchy, &containers);
    }
}

pub fn get_items<'a>(parent: Entity, hierarchy: &ParentHierarchy,
                     container_storage: &WriteStorage<'a, components::Container>,
                     item_storage: &WriteStorage<'a, components::Item>
    )
                          -> BitSet
{
    let mut items = get_containers(parent, hierarchy, container_storage);
    for (_, id) in (item_storage, hierarchy.all_children(parent)).join() {
        items.add(id);
    }

    items
}

pub fn get_containers<'a>(parent: Entity, hierarchy: &ParentHierarchy,
                                       container_storage: &WriteStorage<'a, components::Container>)
    -> BitSet
{
    let mut containers = BitSet::new();
    // Find all items which are a child
    for (_, id) in (container_storage, hierarchy.all_children(parent)).join() {
        containers.add(id);
    }

    containers
}