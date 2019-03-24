#![allow(clippy::module_name_repetitions)]

use crate::actions::Action;
use crate::components;
use crate::settings::Context;
use crate::utils::ComponentEventReader;
use amethyst::{
    assets::AssetStorage,
    core::components::Parent,
    core::transform::Transform,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, Resources, SystemData, WriteStorage},
};

use slog::slog_error;

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, Action>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        ReadStorage<'s, components::Item>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, components::FlaggedSpriteRender>,
        WriteStorage<'s, components::Actionable>,
        WriteStorage<'s, Parent>,
        Read<'s, AssetStorage<crate::assets::Item>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.action_reader.setup(res);
    }

    fn run(
        &mut self,
        (
            context,
            entities,
            items,
            mut transforms,
            mut sprites,
            mut actionables,
            mut parents,
            item_storage,
        ): Self::SystemData,
    ) {
        self.action_reader.maintain(&entities, &mut actionables);

        // search for dropped items with a transform, but no sprite. Add the sprite.
        // This is just to cover our ass for wierd cases...right?

        let sheet_handle = context.spritesheet.as_ref().unwrap();
        for (entity, item, _) in (&entities, &items, &transforms).join() {
            // If the item doesn't have a sprite, add it
            if sprites.get(entity).is_none() {
                // Insert a sprite for it
                if let Some(details) = item_storage.get(&item.handle) {
                    sprites
                        .insert(
                            entity,
                            components::FlaggedSpriteRender {
                                sprite_sheet: sheet_handle.clone(),
                                sprite_number: details.sprite_number,
                            },
                        )
                        .unwrap();
                }
            }
        }

        // check for pickup and drop events
        for (entity, actionable) in (&entities, &mut actionables).join() {
            for event in self.action_reader.read(entity, actionable) {
                match event {
                    Action::Drop(dropped_item) => {
                        // Add a transform based off the parent. and add a sprite.
                        match transforms.get(entity) {
                            Some(transform) => {
                                let item = items.get(entity).unwrap();
                                if let Some(details) = item_storage.get(&item.handle) {
                                    transforms.insert(*dropped_item, transform.clone()).unwrap();
                                    sprites
                                        .insert(
                                            entity,
                                            components::FlaggedSpriteRender {
                                                sprite_sheet: sheet_handle.clone(),
                                                sprite_number: details.sprite_number,
                                            },
                                        )
                                        .unwrap();

                                    // Remove the parent relationship
                                    parents.remove(*dropped_item);
                                }
                            }
                            None => {
                                slog_error!(context.logs.root, "Entity without transform dropped an item, this shouldn't happen!");
                                continue;
                            }
                        }
                    }
                    Action::DoPickup(picked_item) => {
                        transforms.remove(*picked_item);
                        sprites.remove(*picked_item);

                        parents.insert(*picked_item, Parent { entity }).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
