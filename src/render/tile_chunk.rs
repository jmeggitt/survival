use std::mem::size_of;

use amethyst::assets::{AssetStorage, Handle};
use amethyst::core::math::Matrix4;
use amethyst::core::GlobalTransform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Join, ReadStorage, WriteStorage};
use amethyst::renderer::pipe::pass::{Pass, PassData};
use amethyst::renderer::Camera;
use amethyst::renderer::{
    DepthMode, Effect, Encoder, Factory, NewEffect, Resources, Texture, VertexFormat,
};
use amethyst::Error;
use gfx::buffer::Role::Vertex;
use gfx::handle::RawBuffer;
use gfx::memory::{Bind, Typed};
use gfx::pso::buffer::ElemStride;
use glsl_layout::Uniform;
use log::error;
use shred_derive::SystemData;
use specs_derive::Component;

use crate::render::flat_specs::{SpriteInstance, FRAG_SRC, VERT_SRC};
use crate::render::tile_chunk::ChunkRender::Buffered;

use super::flat_specs::{TextureOffsetPod, ViewArgs};

type GraphicsSlice = gfx::Slice<Resources>;

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
    fn buffer(&mut self, factory: &mut Factory) {
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

            point_buffer.shrink_to_fit();
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

#[derive(SystemData)]
pub struct RenderData<'a> {
    chunks: WriteStorage<'a, ChunkRender>,
    camera: ReadStorage<'a, Camera>,
    global: ReadStorage<'a, GlobalTransform>,
    sprite_assets: Read<'a, AssetStorage<Texture>>,
}

impl<'a> RenderData<'a> {
    pub fn camera(&'a self) -> Option<(&'a Camera, &'a GlobalTransform)> {
        (&self.camera, &self.global).join().next()
    }
}

#[allow(dead_code)]
pub struct TileRenderPass;

impl<'a> PassData<'a> for TileRenderPass {
    type Data = RenderData<'a>;
}

impl Pass for TileRenderPass {
    fn compile(&mut self, effect: NewEffect) -> Result<Effect, Error> {
        effect
            .simple(VERT_SRC, FRAG_SRC)
            .without_back_face_culling()
            .with_raw_constant_buffer("ViewArgs", size_of::<<ViewArgs as Uniform>::Std140>(), 1)
            .with_raw_vertex_buffer(
                SpriteInstance::attributes(),
                SpriteInstance::size() as ElemStride,
                1,
            )
            .with_texture("albedo")
            .with_raw_constant_buffer(
                "AlbedoOffset",
                size_of::<<TextureOffsetPod as Uniform>::Std140>(),
                1,
            )
            .with_output("color", Some(DepthMode::LessEqualWrite))
            .build()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        mut factory: Factory,
        mut data: RenderData,
    ) {
        set_view_args(encoder, effect, data.camera());

        for chunk in (&mut data.chunks).join() {
            chunk.buffer(&mut factory);
            if let Buffered(buffer, command) = chunk {
                effect.data.vertex_bufs.push(buffer.clone());

                for (sprite_handle, slice) in command {
                    let texture = match data.sprite_assets.get(sprite_handle) {
                        Some(v) => v,
                        None => {
                            error!("Missing texture {:?}", sprite_handle);
                            continue;
                        }
                    };
                    effect.data.textures.push(texture.view().clone());
                    effect.data.samplers.push(texture.sampler().clone());
                    effect.draw(slice, encoder);
                }
            }
        }
    }
}

pub fn set_view_args(
    encoder: &mut Encoder,
    effect: &mut Effect,
    camera: Option<(&Camera, &GlobalTransform)>,
) {
    let view_args = match camera {
        Some((cam, GlobalTransform(transform))) => {
            let proj: [[f32; 4]; 4] = cam.proj.into();
            let view: [[f32; 4]; 4] = transform
                .try_inverse()
                .expect("Unable to get inverse of camera transform")
                .into();
            ViewArgs::new(proj.into(), view.into())
        }
        None => {
            let identity: [[f32; 4]; 4] = Matrix4::identity().into();
            ViewArgs::new(identity.into(), identity.into())
        }
    };
    effect.update_constant_buffer("ViewArgs", &view_args.std140(), encoder);
}
