use amethyst::{
    StateEvent, Trans, StateData,
};

use crate::SurvivalData;

pub struct State;
impl<'a, 'b> amethyst::State<SurvivalData<'a, 'b>, StateEvent> for State {
    fn handle_event(
        &mut self,
        _: StateData<'_, SurvivalData<'_, '_>>,
        _: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}