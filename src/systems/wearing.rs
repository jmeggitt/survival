#![allow(clippy::module_name_repetitions)]

use amethyst::{
    core::{ParentHierarchy, Parent},
    ecs::{Join, ReadStorage, WriteStorage, Read, ReadExpect, Entity, Entities},
};
use crate::settings::Context;
use crate::components;

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
        ReadExpect<'s, ParentHierarchy>,
        Entities<'s>,
        WriteStorage<'s, components::Item>,
        WriteStorage<'s, Parent>,

    );

    fn run(&mut self, _: Self::SystemData) {

    }
}