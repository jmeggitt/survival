#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{Component, Join,
          DenseVecStorage,
          WriteStorage, Write, ReadExpect},
};
use specs_derive::Component;
use crate::settings::Context;

use slog::slog_info;

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
    pub next_time: u64,
}
impl Default for TimeState {
    fn default() -> Self {
        Self {
            turn: TimeTurn::Player,
            current_time: 0,
            next_time: 0,
        }
    }
}

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, TimeState>,
        WriteStorage<'s, TimeAvailable>,
    );

    fn run(&mut self, (context, mut time_state, mut time_avialables): Self::SystemData) {
        if time_state.current_time < time_state.next_time && time_state.turn == TimeTurn::AI {
            let delta = time_state.next_time - time_state.current_time;
            slog_info!(context.logs.root, "inc={}", delta);
            for mut ta in (&mut time_avialables).join() {
                ta.available_time += delta;
                slog_info!(context.logs.root, "avail={}", ta.available_time);
            }

            time_state.current_time += time_state.next_time;
        }
    }
}