#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{Resources, ReadExpect, WriteStorage, SystemData, Read},
    shrev::{EventChannel, ReaderId},
    ui::{UiText, UiFinder},
    utils::fps_counter::FPSCounter,
};
use crate::settings::Context;
use std::sync::Arc;
use amethyst_imgui::imgui as imgui;

pub type ImguiDraw = Arc<Fn(&imgui::Ui) + Send + Sync>;

#[derive(Default)]
pub struct System {
    draw_call_reader_id: Option<ReaderId<ImguiDraw>>,
}
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Read<'s, EventChannel<ImguiDraw>>,

        Read<'s, FPSCounter>,

        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn run(&mut self, (_, imgui_draw_events, fps, mut texts, finder): Self::SystemData) {
        if let Some(fps_entity) = finder.find("fps") {
            if let Some(fps_display) = texts.get_mut(fps_entity) {
                fps_display.text = format!("FPS: {:.2}", fps.sampled_fps());
            }
        }

        // Get the current ui
        let ui;
        unsafe {
            ui = imgui::Ui::current_ui();
        }
        if let Some(ui) = ui {
            for draw_call in imgui_draw_events.read(self.draw_call_reader_id.as_mut().unwrap()) {
                (draw_call)(ui)
            }
        }


    }
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.draw_call_reader_id = Some(res.fetch_mut::<EventChannel<ImguiDraw>>().register_reader());


        //let mut creator = UiCreator::default();
        //let mut creator: UiCreator<'_> = SystemData::fetch(res);
        //creator.create("ui/example.ron", ());
    }

}