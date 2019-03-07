#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{Component, Resources, SystemData, Join, ReadStorage, WriteStorage, ReadExpect, Entity, Entities},
    core::transform::Transform,
    core::components::Parent,
};
use crate::settings::Context;
use crate::components;
use crate::utils::{ComponentEventReader, HasChannel};
use crate::actions::Action;
use crate::actions;
use slog::slog_error;

use crate::tiles::{Tiles, TileEntities, ReadTiles};

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, Action, <components::Actionable as Component>::Storage>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, crate::settings::Config>,
        ReadExpect<'s, Tiles>,
        Entities<'s>,
        ReadStorage<'s, components::Item>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, components::Actionable>,
        ReadStorage<'s, Parent>,
        ReadTiles<'s, TileEntities>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.action_reader.setup(res);
    }

    fn run(&mut self, (context, config, tiles, entities, items, transforms, mut actionables, parents, tile_entities_map): Self::SystemData) {
        self.action_reader.maintain(&entities, &mut actionables);
        let mut events = Vec::new();
        for (entity, transform, actionable) in (&entities, &transforms, &mut actionables, ).join() {
            for event in self.action_reader.read(entity, actionable) {
                if let Action::TryPickup(target) = event {
                    match target {
                        actions::PickupTarget::Under => {
                            // Target there any other tile entities underneath us?
                            for entity in tile_entities_map.get(tiles.id_from_vector(tiles.world_to_tile(transform.translation(), &config))).unwrap().0.iter() {
                                if let Some(item) = items.get(*entity) {
                                    // Its an item! We can get it.
                                    // TODO: allllll sorts of checks
                                    // rebroadcast the DoPickup event
                                    events.push((*entity, Action::DoPickup(*entity)));
                                }
                            }
                        },
                        actions::PickupTarget::Location(vector) => { slog_error!(context.logs.root, "Location Not implemented"); },
                        actions::PickupTarget::Entity(entity) => { slog_error!(context.logs.root, "Entity Not implemented"); },
                    }
                }
            }
        }

        // Emit all our generated events
        for event in events {
            if let Some(actionable) = actionables.get_mut(event.0) {
                actionable.channel_mut().single_write(event.1);
            }
        }
    }
}