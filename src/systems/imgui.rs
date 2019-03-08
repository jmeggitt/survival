#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{Write, ReadExpect,},
};
use amethyst_imgui as am_imgui;
use amethyst_imgui::imgui as imgui;
use crate::settings::Context;

#[derive(Default)]
pub struct BeginFrameSystem;
impl BeginFrameSystem {
    pub fn open_frame<'ui>(
        &mut self,
        dimensions: &amethyst::renderer::ScreenDimensions,
        time: &amethyst::core::timing::Time,
        imgui_state: &mut Option<am_imgui::ImguiState>,
    ) -> Option<&'ui imgui::Ui<'ui>>
    {
        let dimensions: &amethyst::renderer::ScreenDimensions = &dimensions;
        let time: &amethyst::core::timing::Time = &time;

        if dimensions.width() <= 0. || dimensions.height() <= 0. {
            return None;
        }

        let imgui = match imgui_state {
            Some(x) => &mut x.imgui,
            _ => return None,
        };

        let frame = imgui.frame(imgui::FrameSize::new(f64::from(dimensions.width()), f64::from(dimensions.height()), 1.), time.delta_seconds());
        std::mem::forget(frame);
        unsafe { imgui::Ui::current_ui() }
    }
}
impl<'s> amethyst::ecs::System<'s> for BeginFrameSystem {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, amethyst::renderer::ScreenDimensions>,
        ReadExpect<'s, amethyst::core::timing::Time>,
        Write<'s, Option<am_imgui::ImguiState>>,
    );

    fn run(&mut self, (_, dimensions, time, mut imgui_state, ): Self::SystemData) {
        self.open_frame(&dimensions, &time, &mut imgui_state);
    }
}

#[derive(Default)]
pub struct EndFrameSystem;
impl<'s> amethyst::ecs::System<'s> for EndFrameSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        unsafe {
            if let Some(ui) = imgui::Ui::current_ui() {
                (ui as *const imgui::Ui).read_volatile();
                ui.show_demo_window(&mut true);
            }
        };
    }
}



struct ImguiLuaWrapper<'ui>(&'ui imgui::Ui<'ui>);
impl<'ui> rlua::UserData for ImguiLuaWrapper<'ui> {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("new_line", | _, im, ()| {
            im.0.new_line();
            Ok(())
        });
    }
}
