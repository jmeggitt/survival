#![allow(clippy::module_name_repetitions)]
use amethyst::{
    ecs::{Component, Join,
          Resources, SystemData,
          DenseVecStorage,
          WriteStorage, Write, ReadExpect, WriteExpect},
};
use specs_derive::Component;
use amethyst_imgui as am_imgui;
use amethyst_imgui::imgui as imgui;
use crate::settings::Context;

#[derive(Component, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct ImguiWindow {
    pub script: String,
}

#[derive(Default)]
pub struct System {
    lua: rlua::Lua, // Imgui system stores its own lua context for executing UI scripts with imgui in scope
}

impl System {
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

    pub fn close_frame(&mut self, ui: &imgui::Ui) {
        unsafe {
            (ui as *const imgui::Ui).read_volatile();
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

impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, amethyst::renderer::ScreenDimensions>,
        ReadExpect<'s, amethyst::core::timing::Time>,
        Write<'s, Option<am_imgui::ImguiState>>,
        WriteExpect<'s, crate::systems::script::ScriptRuntime>,
        // storages
        WriteStorage<'s, ImguiWindow>,

    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);


    }

    fn run(&mut self, (_, dimensions, time, mut imgui_state, _, mut windows, ): Self::SystemData) {
        let ui = self.open_frame(&dimensions, &time, &mut imgui_state);

        // Run our shit
        if let Some(ui) = ui {
            for _ in (&mut windows).join() {
                // Run it

            }

            // End of frame
            self.close_frame(ui);
        }
    }
}