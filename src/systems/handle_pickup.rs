use amethyst::{
    core::components::Parent,
    core::transform::Transform,
    ecs::{Entities, Join, ReadExpect, ReadStorage, Resources, SystemData, WriteStorage},
};
use log::error;

use crate::actions;
use crate::actions::Action;
use crate::components;
use crate::tiles::{ReadTiles, TileEntities, Tiles};
use crate::utils::{ComponentEventReader, HasChannel};

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, Action>,
}

impl<'s> amethyst::ecs::System<'s> for System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
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

    fn run(
        &mut self,
        (
            config,
            tiles,
            entities,
            items,
            transforms,
            mut actionables,
            _,
            tile_entities_map,
        ): Self::SystemData,
    ) {
        self.action_reader.maintain(&entities, &mut actionables);
        let mut events = Vec::new();
        for (entity, transform, actionable) in (&entities, &transforms, &mut actionables).join() {
            for event in self.action_reader.read(entity, actionable) {
                if let Action::TryPickup(target) = event {
                    match target {
                        actions::PickupTarget::Under => {
                            // Target there any other tile entities underneath us?
                            for entity in &tile_entities_map
                                .get(tiles.id_from_vector(
                                    tiles.world_to_tile(transform.translation(), &config),
                                ))
                                .unwrap()
                                .0
                            {
                                if items.get(*entity).is_some() {
                                    // Its an item! We can get it.
                                    // TODO: allllll sorts of checks
                                    // rebroadcast the DoPickup event
                                    events.push((*entity, Action::DoPickup(*entity)));
                                }
                            }
                        }
                        actions::PickupTarget::Location(_) => {
                            error!("Location Not implemented");
                        }
                        actions::PickupTarget::Entity(_) => {
                            error!("Entity Not implemented");
                        }
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
