use amethyst::{
    ecs::{Join, ReadStorage, ReadExpect, Entity, Entities},
};

use crate::settings::Context;

#[derive(Default)]
pub struct System;

impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        Entities<'s>,
    );

    fn run(&mut self, _: Self::SystemData) {}
}