use crate::chunk::Chunk;
use crate::specs_static::{Id, Storage};
use amethyst::assets::Handle;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Component, Write};
use amethyst::renderer::Texture;
use hashbrown::HashMap;
use specs_derive::Component;

mod pass;
mod specs;

pub use pass::TileRenderPass;
