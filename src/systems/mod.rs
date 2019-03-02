use amethyst::{
    ecs::{Component, System, Join,
          DenseVecStorage,
          WriteStorage, ReadStorage, Write, Read},
    input::InputHandler,
};
use specs_derive::Component;
use crate::components::Player;

#[derive(Component, Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[storage(DenseVecStorage)]
pub struct TimeAvailable {
    pub available_time: u64,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TimeTurn {
    Player,
    AI,
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeState {
    pub turn: TimeTurn,
    pub current_time: u64,
    pub player_next_time: u64,
}
impl Default for TimeState {
    fn default() -> Self {
        Self {
            turn: TimeTurn::Player,
            current_time: 0,
            player_next_time: 0,
        }
    }
}

#[derive(Default)]
pub struct TimeSystem;
impl<'s> System<'s> for TimeSystem {
    type SystemData = (
        Write<'s, TimeState>,
        WriteStorage<'s, TimeAvailable>,
    );

    fn run(&mut self, (mut time_state, mut time_avialables): Self::SystemData) {
        if time_state.current_time < time_state.player_next_time && time_state.turn == TimeTurn::AI {
            let delta = time_state.player_next_time - time_state.current_time;
            for mut ta in (&mut time_avialables).join() {
                ta.available_time += delta;
            }

            time_state.current_time += time_state.player_next_time;
        }
    }
}

#[derive(Default)]
pub struct ActionSystem;
impl<'s> System<'s> for ActionSystem {
    type SystemData = (
    );

    fn run(&mut self, (): Self::SystemData) {

    }
}

#[derive(Default)]
pub struct PlayerInputSystem;
impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (_, _): Self::SystemData) {
        //let x_move = input.axis_value("entity_x").unwrap();
        //let y_move = input.axis_value("entity_y").unwrap();

        //for (_, transform) in (&players, &mut transforms).join() {
        //    transform.translate_x(x_move as f32 * 20.0);
        //    transform.translate_y(y_move as f32 * 20.0);
        //}
    }
}