#![allow(clippy::module_name_repetitions)]

use amethyst::{
    core::{ParentHierarchy},
    ecs::{Join, Entity, storage::{GenericReadStorage},},
};
use crate::components;
use hibitset::BitSet;

pub fn in_container<S>(item: Entity, hierarchy: &ParentHierarchy,
                    container_storage: &S) -> bool
    where S: GenericReadStorage<Component=components::Container>
{
    if let Some(e) = hierarchy.parent(item) {
        return container_storage.get(e).is_some();
    }
    false
}

pub fn get_items_in_container<S>(container: Entity, hierarchy: &ParentHierarchy,
                         item_storage: S
) -> BitSet
    where S: GenericReadStorage<Component=components::Item> + Join
{
    let mut items = BitSet::new();

    (item_storage, hierarchy.all_children(container)).join().for_each(|(_, id)| {
        items.add(id);
    });

    items
}

pub fn get_all_items<C, I>(parent: Entity, hierarchy: &ParentHierarchy,
                     container_storage: C,
                     item_storage: I
) -> BitSet
    where I: GenericReadStorage<Component=components::Item> + Join,
          C: GenericReadStorage<Component=components::Container> + Join
{
    let mut items = get_all_containers(parent, hierarchy, container_storage);
    (item_storage, hierarchy.all_children(parent)).join().for_each(|(_, id)| {
        items.add(id);
    });

    items
}

pub fn get_all_containers<S>(parent: Entity, hierarchy: &ParentHierarchy,
                          container_storage: S
) -> BitSet
    where S: GenericReadStorage<Component=components::Container> + Join
{
    let mut containers = BitSet::new();
    // Find all items which are a child
    (container_storage, hierarchy.all_children(parent)).join().for_each(|(_, id)| {
        containers.add(id);
    });

    containers
}
