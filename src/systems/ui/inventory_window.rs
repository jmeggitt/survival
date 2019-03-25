#![allow(clippy::module_name_repetitions)]

use amethyst::{
    assets::AssetStorage,
    core::ParentHierarchy,
    ecs::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, Resources, SystemData, Write,
        WriteStorage,
    },
    input::InputEvent,
    renderer::HiddenPropagate,
    shrev::{EventChannel, ReaderId},
    ui::{UiCreator, UiFinder, UiText},
};

use crate::actions::PlayerInputAction;
use crate::assets;
use crate::components;
use crate::settings::Context;

#[derive(Default)]
pub struct System {
    main_ui: Option<Entity>,
    inventory: Option<Entity>,
    input_reader_id: Option<ReaderId<InputEvent<PlayerInputAction>>>,
}

impl<'s> amethyst::ecs::System<'s> for System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        Read<'s, EventChannel<InputEvent<PlayerInputAction>>>,
        ReadExpect<'s, ParentHierarchy>,
        WriteStorage<'s, components::Item>,
        WriteStorage<'s, components::Container>,
        WriteStorage<'s, HiddenPropagate>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, components::Player>,
        Read<'s, AssetStorage<assets::Item>>,
        UiFinder<'s>,
    );

    fn run(
        &mut self,
        (
            _,
            entities,
            input_events,
            hierarchy,
            item_storage,
            container_storage,
            mut hidden_storage,
            mut text_storage,
            player_storage,
            item_details,
            finder,
        ): Self::SystemData,
    ) {
        for player_input in input_events.read(self.input_reader_id.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(PlayerInputAction::ToggleInventory) = player_input {
                if let Some(inventory) = finder.find("inventory_window") {
                    if hidden_storage.contains(inventory) {
                        hidden_storage.remove(inventory);
                    } else {
                        hidden_storage.insert(inventory, HiddenPropagate).unwrap();
                    }
                }
            }
        }

        if let Some(inventory_text) = finder.find("inventory_text") {
            if let Some(inventory_display) = text_storage.get_mut(inventory_text) {
                let (_, player) = (&player_storage, &entities).join().next().unwrap();

                inventory_display.text = crate::inventory::draw_inventory(
                    player,
                    &hierarchy,
                    container_storage,
                    &item_storage,
                    &item_details,
                );
            }
        }
    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.input_reader_id = Some(
            Write::<EventChannel<InputEvent<PlayerInputAction>>>::fetch(&res).register_reader(),
        );

        let mut creator: UiCreator<'_> = SystemData::fetch(res);
        self.inventory = Some(creator.create("ui/inventory.ron", ()));
    }
}
