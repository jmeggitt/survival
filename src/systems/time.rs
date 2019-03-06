#![allow(clippy::module_name_repetitions)]

use amethyst::{
    ecs::{ Join, ReadStorage, WriteStorage, Write, ReadExpect, Entity, Entities},
};
use crate::settings::Context;
use crate::components;

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
        Entities<'s>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::TimeAvailable>,
    );

    fn run(&mut self, (_, mut time_state,
                        entities, players, mut time_avialables): Self::SystemData) {
        // If the player has time available, it means they did an action.
        // Add it to the resource and consume it
        let mut time_consumed = 0;

        for (_, _, time_comp) in (&entities, &players, &mut time_avialables).join() {
            if time_comp.0 > 0 {
                // Player did time.
                //slog_trace!(context.logs.root, "Player performed action consuming {} time", time_comp.0);
                time_consumed = time_comp.0;
                time_state.current_time += time_consumed;

                time_comp.consume(time_comp.0);
            }
        }

        // Update all time comps with the time consumed
        if time_consumed > 0 {
           // slog_trace!(context.logs.root, "Adding time available to all AI: {}", time_consumed);
            for (entity, time_comp) in (&entities, &mut time_avialables).join() {
                if players.get(entity).is_none() {
                    time_comp.add(time_consumed);
                }
            }
        }

    }
}

pub fn has_time(time: u64, entity: Entity, time_comp: &mut components::TimeAvailable, players: &ReadStorage<components::Player>, ) -> bool {
    if players.get(entity).is_some() {
        // Is player
        true
    } else {
        // Is AI
        time_comp.has(time)
    }
}

pub fn consume_time(time: u64, entity: Entity, time_comp: &mut components::TimeAvailable, players: &ReadStorage<components::Player>, ) {
    if players.get(entity).is_some() {
        // Is player
        time_comp.add(time);
    } else {
        // Is AI
        time_comp.consume(time);
    }
}