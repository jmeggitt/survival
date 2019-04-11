use amethyst::assets::Handle;
use amethyst::ecs::{Component, Write};
use amethyst::ecs::prelude::*;
use amethyst::renderer::Texture;
use hashbrown::HashMap;
pub use pass::TileRenderPass;
use specs_derive::Component;

use crate::chunk::Chunk;
use crate::specs_static::{Id, Storage};

mod pass;
mod specs;

