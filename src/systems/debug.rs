#![allow(clippy::module_name_repetitions)]
use std::sync::Arc;
use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, Entity, Entities, Write, Read},
    shrev::EventChannel,
};
use crate::settings::Context;
use crate::systems::ui::ImGuiDraw;
use crate::assets;

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        Write<'s, EventChannel<ImGuiDraw>>,

        Read<'s, assets::ItemStorage>
    );

    fn run(&mut self, (_, _, mut imgui_draw, item_storage): Self::SystemData) {
        use amethyst_imgui::imgui as imgui;
        use assets::GetStorage;
        use amethyst_imgui::imgui::{ImString, im_str};
        use std::borrow::Borrow;

        let items = item_storage.read().unwrap().data.keys().map(|k| { ImString::new(k.as_str()) }).collect::<Vec<_>>();

        imgui_draw.single_write(Arc::new(move |ui: &amethyst_imgui::imgui::Ui| {
            ui.window(imgui::im_str!("Item Explorer"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    let mut refs = vec![];
                    for item in items.iter() {
                        refs.push(item.borrow());
                    }
                    let mut current_item = 0;
                    ui.list_box(
                        im_str!("Items"),
                        &mut current_item,
                        refs.as_slice(),
                        10,
                    );
                })
        }));
    }
}