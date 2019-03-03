use specs_derive::Component;
use amethyst::ecs::prelude::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use amethyst_imgui as am_imgui;
use amethyst_imgui::imgui as imgui;

#[derive(Component, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct ImguiWindow {
}

pub fn start_update<'ui>(world: &'ui mut World) -> Option<&'ui imgui::Ui<'ui>> {
    type Data<'system_data> = (
        WriteStorage<'system_data, ImguiWindow>,
    );

    let ui = amethyst_imgui::open_frame(world);
    if let Some(ui) = ui {
        ui.show_demo_window(&mut true);

        //
        //world.write_resource();

        // Run windows
        let (mut windows) = Data::fetch(&world.res);
        for (window) in (&mut windows.0).join() {

        }
    }

    ui
}

pub fn end_update<'ui>(ui: &imgui::Ui<'ui>) {
    am_imgui::close_frame(ui)
}