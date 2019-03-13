#![allow(clippy::module_name_repetitions)]
use std::sync::Arc;
use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, Entity, Entities, Write},
    shrev::EventChannel,
};
use crate::settings::Context;
use crate::systems::ui::ImGuiDraw;

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
        Write<'s, EventChannel<ImGuiDraw>>,
    );

    fn run(&mut self, (_, _, mut imgui_draw): Self::SystemData) {
        imgui_draw.single_write(Arc::new(|ui: &amethyst_imgui::imgui::Ui| {
            use amethyst_imgui::imgui as imgui;

            ui.window(imgui::im_str!("Map Gen Test"))
            .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    ui.text(imgui::im_str!("Hello world!"));
                })
        }));
    }
}