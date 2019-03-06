#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{Component, Join,
          DenseVecStorage,
          WriteStorage, Write, ReadExpect},
    shrev::{EventChannel}
};
use specs_derive::Component;
use crate::settings::Context;
use crate::components::EnergyAvailable;

use slog::slog_info;

#[derive(Default, Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeState {
    pub current_time: u64,
}


#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Write<'s, TimeState>,
        WriteStorage<'s, EnergyAvailable>,
    );

    fn run(&mut self, (context, mut time_state, mut time_avialables): Self::SystemData) {

    }
}