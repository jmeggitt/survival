use amethyst::{StateData, StateEvent, Trans};

use crate::game_data::SurvivalState;
use crate::SurvivalData;

pub struct Running;

impl<'a, 'b> amethyst::State<SurvivalData<'a, 'b>, StateEvent> for Running {
    fn on_start(&mut self, _: StateData<'_, SurvivalData<'_, '_>>) {}

    fn on_pause(&mut self, _: StateData<'_, SurvivalData<'_, '_>>) {}

    fn handle_event(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        amethyst_imgui::handle_imgui_events(data.world, &event);
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        if data.data.update(&data.world, SurvivalState::Running) != SurvivalState::Running {
            return Trans::Pop;
        }

        Trans::None
    }
}
