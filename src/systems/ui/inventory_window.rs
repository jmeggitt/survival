#![allow(clippy::module_name_repetitions)]

use amethyst::{
    assets::AssetStorage,
    renderer::HiddenPropagate,
    core::{ParentHierarchy},
    ecs::{Entity, Entities, Resources, ReadExpect, Write, WriteStorage, SystemData, Read, Join, ReadStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiText, UiFinder, UiCreator},
    utils::fps_counter::FPSCounter,
    input::{InputEvent},
};
use crate::settings::Context;
use crate::actions::PlayerInputAction;
use crate::components;
use crate::inventory;
use crate::assets;

use super::ImGuiDraw;

#[derive(Default)]
pub struct System {
    main_ui: Option<Entity>,
    inventory: Option<Entity>,
    input_reader_id: Option<ReaderId<InputEvent<PlayerInputAction>>>,
}

impl<'s> amethyst::ecs::System<'s> for System {
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

    fn run(&mut self, (_, entities, input_events, hierarchy,
        mut item_storage, container_storage,
        mut hidden_storage, mut text_storage,
        player_storage,
        item_details,
        finder)
    : Self::SystemData) {
        for player_input in input_events.read(self.input_reader_id.as_mut().unwrap()) {
            match player_input {
                InputEvent::ActionPressed(PlayerInputAction::ToggleInventory) => {
                    if let Some(inventory) = finder.find("inventory_window") {
                        if hidden_storage.contains(inventory) {
                            hidden_storage.remove(inventory);
                        } else {
                            hidden_storage.insert(inventory, HiddenPropagate).unwrap();
                        }
                    }
                },
                _ => {},
            }
        }

        if let Some(inventory_text) = finder.find("inventory_text") {
            if let Some(inventory_display) = text_storage.get_mut(inventory_text) {
                let (_, player) = (&player_storage, &entities).join().next().unwrap();

                inventory_display.text = crate::inventory::draw_inventory(player,
                                                                          &entities,
                                                                          &hierarchy,
                                                                          container_storage,
                                                                          item_storage,
                                                                          &item_details);
            }
        }

    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.input_reader_id = Some(Write::<EventChannel<InputEvent<PlayerInputAction>>>::fetch(&res).register_reader());

        let mut creator: UiCreator<'_> = SystemData::fetch(res);
        self.inventory = Some(creator.create("ui/inventory.ron", ()));
    }

}