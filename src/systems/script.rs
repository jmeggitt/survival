#![allow(clippy::module_name_repetitions)]
use std::sync::Arc;
use std::collections::HashMap;
use amethyst::{
    ecs::{Component, Join,
          DenseVecStorage,
          WriteStorage, Write, ReadExpect},
};
use specs_derive::Component;

use crate::settings::Context;

#[derive(Default)]
pub struct ScriptRuntime {

}

#[derive(Default)]
pub struct System;
impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (context): Self::SystemData) {

    }
}