use std::mem::size_of;

use amethyst::assets::AssetStorage;
use amethyst::core::math::Matrix4;
use amethyst::core::GlobalTransform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Join, ReadStorage};
use amethyst::renderer::pipe::pass::{Pass, PassData};
use amethyst::renderer::Camera;
use amethyst::renderer::{Effect, Encoder, Factory, NewEffect, Resources, Texture, VertexFormat};
use amethyst::Error;
use gfx::buffer::Role::Vertex;
use gfx::memory::{Bind, Typed};
use gfx::pso::buffer::ElemStride;
use glsl_layout::Uniform;
use log::warn;
use shred_derive::SystemData;

use super::specs::{SpriteInstance, TextureOffsetPod, ViewArgs, FRAG_SRC, VERT_SRC};
use super::WriteChunkRender;

type GraphicsSlice = gfx::Slice<Resources>;

#[derive(SystemData)]
pub struct RenderData<'a> {
    chunks: WriteChunkRender<'a>,
    camera: ReadStorage<'a, Camera>,
    global: ReadStorage<'a, GlobalTransform>,
    sprite_assets: Read<'a, AssetStorage<Texture>>,
}

impl<'a> RenderData<'a> {
    pub fn camera(&'a self) -> Option<(&'a Camera, &'a GlobalTransform)> {
        (&self.camera, &self.global).join().next()
    }
}

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
            .with_output("color", None)
            .build()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        mut factory: Factory,
        mut data: RenderData<'b>,
    ) {
        set_view_args(encoder, effect, data.camera());

        use gfx::Factory;
        for chunk in (&mut data.chunks).join() {
            for usage in chunk.inner.iter() {
                // Get texture
                match data.sprite_assets.get(&usage.texture) {
                    Some(tex) => {
                        effect.data.textures.push(tex.view().clone());
                        effect.data.samplers.push(tex.sampler().clone());
                    }
                    None => {
                        warn!("Missing texture {:?}", &usage.texture);
                        continue;
                    }
                };

                match factory.create_buffer_immutable(&usage.data, Vertex, Bind::empty()) {
                    Ok(v) => effect.data.vertex_bufs.push(v.raw().clone()),
                    Err(_) => {
                        warn!("Unable to create immutable graphics buffer");
                        effect.clear();
                        continue;
                    }
                };

                let slice = GraphicsSlice {
                    start: 0,
                    end: 6,
                    base_vertex: 0,
                    instances: Some((usage.len, 0)),
                    buffer: Default::default(),
                };

                effect.draw(&slice, encoder);
                effect.clear();
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
