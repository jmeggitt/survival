use std::mem::size_of;

use amethyst::assets::{AssetStorage, Handle};
use amethyst::core::components::Transform;
use amethyst::core::GlobalTransform;
use amethyst::core::math::{Matrix4, Vector4};
use amethyst::ecs::{Join, ReadStorage, WriteStorage};
use amethyst::ecs::{Component, Write};
use amethyst::ecs::prelude::*;
use amethyst::Error;
use amethyst::renderer::{
    DepthMode, Effect, Encoder, Factory, NewEffect, Resources, Texture, VertexFormat,
};
use amethyst::renderer::Camera;
use amethyst::renderer::pipe::pass::{Pass, PassData};
use gfx::buffer::Role::Vertex;
use gfx::handle::RawBuffer;
use gfx::memory::{Bind, Typed};
use gfx::pso::buffer::ElemStride;
use glsl_layout::Uniform;
use hashbrown::HashMap;
use log::warn;
use log::debug;
use shred_derive::SystemData;
use specs_derive::Component;

use crate::chunk::Chunk;
use crate::render::flat_specs::{FRAG_SRC, SpriteInstance, VERT_SRC};
use crate::specs_static::{Id, Storage};
use crate::tiles::TileAsset;
use crate::utils::TILE_SIZE;

use super::flat_specs::{TextureOffsetPod, ViewArgs};

type GraphicsSlice = gfx::Slice<Resources>;

#[derive(Debug)]
pub struct TextureUsage {
    texture: Handle<Texture>,
    data: Vec<f32>,
    len: u32,
}

pub fn compile_chunk(chunk: &Chunk, tile_specs: &[TileAsset]) -> ChunkRender {
    // Compile slice
    let mut texture_map: HashMap<usize, TextureUsage> = HashMap::new();
    for x in 0..16 {
        for y in 0..16 {
            let texture_id = chunk.tiles[x][y].0 as usize;
            let asset = &tile_specs[texture_id];
            let (chunk_x, chunk_y) = chunk.pos;

            // TODO: Use config for tile scale
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                (x as f32 * TILE_SIZE + 16. * chunk_x as f32) * 8.,
                (y as f32 * TILE_SIZE + 16. * chunk_y as f32) * 8.,
                0.,
            );
            transform.set_scale(8., 8., 8.);
            let transform = transform.matrix();

            let dir_x = transform.column(0) * TILE_SIZE;
            let dir_y = transform.column(1) * TILE_SIZE;

            let pos = transform
                * Vector4::new(-asset.sprite.offsets[0], -asset.sprite.offsets[1], 0.0, 1.0);

            let slice = [
                dir_x.x,
                dir_x.y,
                dir_y.x,
                dir_y.y,
                pos.x,
                pos.y,
                asset.sprite.tex_coords.left,
                asset.sprite.tex_coords.right,
                asset.sprite.tex_coords.bottom,
                asset.sprite.tex_coords.top,
            ];

            match texture_map.get_mut(&texture_id) {
                Some(usage) => {
                    usage.data.extend(&slice);
                    usage.len += 1;
                }
                None => {
                    let usage = TextureUsage {
                        texture: asset.texture.clone(),
                        data: slice[..].into(),
                        len: 1,
                    };
                    texture_map.insert(texture_id, usage);
                }
            }
        }
    }

    let mut collected = Vec::with_capacity(texture_map.len());
    for (_, texture_usage) in texture_map {
        collected.push(texture_usage);
    }
    ChunkRender {
        inner: collected
    }
}

pub type WriteChunkRender<'a> =
Write<'a, Storage<ChunkRender, <ChunkRender as Component>::Storage, (i32, i32)>>;

// TODO: Expand to include full i32 range
impl Id for (i32, i32) {
    fn from_u32(value: u32) -> Self {
        ((value & 0xFF) as i8 as i32, (value >> 8) as i8 as i32)
    }

    fn id(&self) -> u32 {
        let (a, b) = self;
        use rand::RngCore;
        use rand::SeedableRng;
        let mut seed = [0u8; 32];
        unsafe {
            rand::rngs::StdRng::from_seed(::std::mem::transmute_copy(&(*a, *b))).next_u32() >> 8
        }
    }
}

#[derive(Debug, Component)]
#[storage(HashMapStorage)]
pub struct ChunkRender {
    inner: Vec<TextureUsage>,
}

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
                    },
                    None => {
                        warn!("Missing texture {:?}", &usage.texture);
                        continue;
                    }
                };

                match factory.create_buffer_immutable(&usage.data, Vertex, Bind::empty()) {
                        Ok(v) => effect.data.vertex_bufs.push(v.raw().clone()),
                        Err(_) => {
                            warn!("Unable to create immutable graphics buffer");
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
