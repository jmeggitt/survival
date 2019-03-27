use std::sync::Arc;

use amethyst::{
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, Resources, SystemData, Write},
    shrev::EventChannel,
};
use amethyst_imgui::imgui::ImString;
use parking_lot::Mutex;

use crate::assets;
use crate::components;
use crate::initializers;
use crate::initializers::SpawnType;
use crate::settings::Context;
use crate::systems::ui::ImGuiDraw;

#[derive(Clone, Default, Debug)]
struct UiEditItemState {
    pub name: ImString,
    pub desc: ImString,
}

#[derive(Clone)]
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
        Read<'s, assets::ItemStorage>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.item_explorer_state = Arc::new(Mutex::new(ItemExplorerUiState::default()));
    }

    fn run(&mut self, (_, _, mut imgui_draw, item_storage): Self::SystemData) {
        use amethyst_imgui::imgui;
        use amethyst_imgui::imgui::{im_str, ImString};
        use std::borrow::Borrow;

        let items = item_storage
            .read()
            .data
            .keys()
            .map(|k| ImString::new(k.as_str()))
            .collect::<Vec<_>>();

        {
            let mut state_lck = self.item_explorer_state.lock();
            if state_lck.last_current_item != state_lck.current_item {
                state_lck.active_item = Some(
                    item_storage
                        .read()
                        .data
                        .values()
                        .nth(state_lck.current_item as usize)
                        .unwrap()
                        .clone(),
                );
                //state_lck.edit_item.name = state_lck.active_item.unwrap().name;
            }
        }

        let state = self.item_explorer_state.clone();

        imgui_draw.single_write(Arc::new(
            move |ui: &amethyst_imgui::imgui::Ui, lazy: &LazyUpdate| {
                let state = state.clone();
                ui.window(imgui::im_str!("Item Explorer"))
                    .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                    .build(|| {
                        let mut refs = vec![];
                        for item in &items {
                            refs.push(item.borrow());
                        }

                        let mut state_lock = state.lock();

                        state_lock.last_current_item = state_lock.current_item;
                        ui.list_box(
                            im_str!("Items"),
                            &mut state_lock.current_item,
                            refs.as_slice(),
                            10,
                        );

                        ui.same_line(0.);
                        if ui.button(im_str!("Spawn"), (0., 0.)) {
                            lazy.exec_mut(move |world| {
                                let (_, player) = (
                                    &world.read_storage::<components::Player>(),
                                    &world.entities(),
                                )
                                    .join()
                                    .next()
                                    .unwrap();

                                initializers::spawn_item(
                                    world,
                                    SpawnType::Parent(player),
                                    "Container",
                                    None,
                                );
                            });
                        }
                    })
            },
        ));
    }
}
