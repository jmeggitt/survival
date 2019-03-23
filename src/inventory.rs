#![allow(clippy::module_name_repetitions)]

use amethyst::{
    assets::AssetStorage,
    core::{ParentHierarchy},
    ecs::{Join, Entities, Entity, storage::{GenericReadStorage}, BitSet, ReadStorage},
};
use crate::assets;
use crate::components;

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
    where S: GenericReadStorage<Component=components::Item> + Copy + Join
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
    where I: GenericReadStorage<Component=components::Item> + Copy + Join,
          C: GenericReadStorage<Component=components::Container> + Copy + Join
{
    let mut items = get_all_containers(parent, hierarchy, container_storage);
    (item_storage, &hierarchy.all_children(parent)).join().for_each(|(_, id)| {
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

pub fn draw_inventory<C, I>(parent: Entity, entities: &Entities, hierarchy: &ParentHierarchy,
                            container_storage: C,
                            item_storage: I,
                            details_storage: &AssetStorage<crate::assets::Item>,
) -> String
    where  C: GenericReadStorage<Component=components::Container>,
           I: GenericReadStorage<Component=components::Item>,

{
    let mut inv = String::new();
    inv += "Inventory:\n";
    hierarchy.children(parent).iter().for_each(|child| {
        println!("Child found");
        if let Some(item) = item_storage.get(*child) {
            println!("item found");
            println!("Handle = {:?}", item.handle);
            if let Some(details) = details_storage.get(&item.handle) {
                println!("details found");
                if let Some(_) = container_storage.get(*child) {
                    println!("Container found");
                    inv += &format!("- {}\t50/100\n", details.name);
                }
            }
        }
    });

    inv
}