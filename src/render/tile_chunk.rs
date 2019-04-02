use crate::chunk::Chunk;
use crate::components::FlaggedSpriteRender;
use crate::tiles::TileId;
use amethyst::assets::{AssetStorage, Handle};
use amethyst::renderer::pipe::pass::{Pass, PassData};
use amethyst::renderer::Texture;
type GraphicsSlice = gfx::Slice<Resources>;
use amethyst::ecs::prelude::*;
use amethyst::ecs::WriteStorage;
use amethyst::renderer::NewEffect;
use amethyst::renderer::{Effect, Encoder, Factory, Resources};
use amethyst::Error;
use gfx::buffer::Role::Vertex;
use gfx::memory::{Bind, Typed};
use gfx::{handle::RawBuffer, Slice};
use log::error;
use specs_derive::Component;

use crate::render::tile_chunk::ChunkRender::Buffered;

#[derive(Debug)]
pub struct TextureUsage {
    texture: Handle<Texture>,
    data: Vec<f32>,
    len: u32,
}

#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub enum ChunkRender {
    Unbuffered(Vec<TextureUsage>),
    Buffered(RawBuffer<Resources>, Vec<(Handle<Texture>, GraphicsSlice)>),
}

impl ChunkRender {
    /// Convert a `ChunkRender::Unbuffered` to a `ChunkRender::Buffered` for easier use with graphics.
    fn buffer(&mut self, mut factory: Factory) {
        use gfx::Factory;
        if let ChunkRender::Unbuffered(texture_usages) = self {
            let mut point_buffer: Vec<f32> = Vec::with_capacity(texture_usages.len() * 30);
            let mut slice_buffer = Vec::with_capacity(texture_usages.len());

            for usage in texture_usages {
                let gfx_slice = GraphicsSlice {
                    start: 0,
                    end: 6,
                    base_vertex: point_buffer.len() as u32,
                    instances: Some((usage.len, 0)),
                    buffer: Default::default(),
                };

                point_buffer.extend(usage.data.iter());
                slice_buffer.push((usage.texture.clone(), gfx_slice))
            }

            let buffer = match factory.create_buffer_immutable(&point_buffer, Vertex, Bind::empty())
            {
                Ok(v) => v,
                Err(_) => {
                    error!("Unable to create immutable graphics buffer");
                    return;
                }
            };

            *self = Buffered(buffer.raw().clone(), slice_buffer)
        }
    }
}

pub struct TileRenderPass;

impl<'a> PassData<'a> for TileRenderPass {
    type Data = (WriteStorage<'a, ChunkRender>);
}

impl Pass for TileRenderPass {
    fn compile(&mut self, effect: NewEffect) -> Result<Effect, Error> {
        unimplemented!()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        factory: Factory,
        data: <Self as PassData<'b>>::Data,
    ) {
        let (mut render_storage) = data;
    }
}
