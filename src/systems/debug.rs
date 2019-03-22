#![allow(clippy::module_name_repetitions)]
use std::sync::{Arc, Mutex};
use amethyst::{
    ecs::{ReadExpect, Entities, Write, Read, Resources, SystemData},
    shrev::EventChannel,
};
use crate::settings::Context;
use crate::systems::ui::ImGuiDraw;
use crate::assets;

use amethyst_imgui::imgui::ImString;

#[derive(Clone, Default, Debug)]
struct UiEditItemState {
    pub name: ImString,
    pub desc: ImString,
}

#[derive(Clone,)]
struct ItemExplorerUiState {
    pub last_current_item: i32,
    pub current_item: i32,
    pub active_item: Option<assets::Item>,
    pub edit_item: UiEditItemState,
}
impl Default for ItemExplorerUiState {
    fn default() -> Self {
        Self {
            last_current_item: 9999,
            current_item: 0,
            active_item: None,
            edit_item: UiEditItemState::default(),
        }
    }
}

#[derive(Default)]
pub struct System {
    item_explorer_state: Arc<Mutex<ItemExplorerUiState>>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        Write<'s, EventChannel<ImGuiDraw>>,

        Read<'s, assets::ItemStorage>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.item_explorer_state = Arc::new(Mutex::new(ItemExplorerUiState::default()));
    }

    fn run(&mut self, (_, _, mut imgui_draw, item_storage): Self::SystemData) {
        use amethyst_imgui::imgui as imgui;
        use amethyst_imgui::imgui::{ImString, im_str};
        use std::borrow::Borrow;

        let items = item_storage.read().unwrap().data.keys().map(|k| { ImString::new(k.as_str()) }).collect::<Vec<_>>();

        {
            let mut state_lck = self.item_explorer_state.lock().unwrap();
            if state_lck.last_current_item != state_lck.current_item {
                state_lck.active_item = Some(item_storage.read().unwrap().data.values().nth(state_lck.current_item as usize).unwrap().clone());
                //state_lck.edit_item.name = state_lck.active_item.unwrap().name;
            }
        }

        let state = self.item_explorer_state.clone();

        imgui_draw.single_write(Arc::new(move |ui: &amethyst_imgui::imgui::Ui| {
            let state = state.clone();
            ui.window(imgui::im_str!("Item Explorer"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    let mut refs = vec![];
                    for item in &items {
                        refs.push(item.borrow());
                    }

                    let mut state_lck = state.lock().unwrap();

                    state_lck.last_current_item = state_lck.current_item;
                    ui.list_box(
                        im_str!("Items"),
                        &mut state_lck.current_item,
                        refs.as_slice(),
                        10,
                    );

                    ui.same_line(0.);
                    if let Some(_) = state_lck.active_item.as_ref() {
                        ui.input_text(im_str!("Seed"), &mut state_lck.edit_item.name).build();
                    }
                })
        }));
    }
}