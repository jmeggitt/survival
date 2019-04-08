//! Flat forward drawing pass that mimics a blit.

use amethyst::assets::{AssetStorage, Handle};
use amethyst::core::ecs::prelude::{Read, ReadExpect, ReadStorage};
use amethyst::core::math::{Vector3, Vector4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::error::Error;
use amethyst::renderer::{
    get_camera,
    pipe::{
        pass::{Pass, PassData},
        DepthMode, Effect, NewEffect,
    },
    ActiveCamera, Attributes, Camera, Color, DisplayConfig, Encoder, Factory, Flipped, Query,
    Resources, Rgba, SpriteSheet, Texture, VertexFormat,
};
use derivative::Derivative;
use gfx::pso::buffer::ElemStride;
use gfx_core::state::{Blend, ColorMask};
use glsl_layout::Uniform;
use log::warn;

use crate::components::FlaggedSpriteRender;
use crate::settings::Config;
use crate::tiles::*;

use super::util::{add_texture, default_transparency, set_view_args, setup_textures, ViewArgs};
use super::*;

type Slice = gfx::Slice<Resources>;

/// Draws sprites on a 2D quad.
#[derive(Derivative, Clone, Debug)]
#[derivative(Default(bound = "Self: Pass"))]
pub struct TileRenderPass {
    #[derivative(Default(value = "default_transparency()"))]
    transparency: Option<(ColorMask, Blend, Option<DepthMode>)>,
    batch: TextureBatch,
    map_transform: Option<GlobalTransform>,
}

impl TileRenderPass {
    /// Create instance of `DrawFlat2D` pass
    pub fn new() -> Self {
        Default::default()
    }

    /// Transparency is enabled by default.
    /// If you pass false to this function transparency will be disabled.
    ///
    /// If you pass true and this was disabled previously default settings will be reinstated.
    /// If you pass true and this was already enabled this will do nothing.
    pub fn with_transparency(mut self, input: bool) -> Self {
        if input {
            if self.transparency.is_none() {
                self.transparency = default_transparency();
            }
        } else {
            self.transparency = None;
        }
        self
    }

    /// Set transparency settings to custom values.
    pub fn with_transparency_settings(
        mut self,
        mask: ColorMask,
        blend: Blend,
        depth: Option<DepthMode>,
    ) -> Self {
        self.transparency = Some((mask, blend, depth));
        self
    }

    fn attributes() -> Attributes<'static> {
        <SpriteInstance as Query<(DirX, DirY, Pos, OffsetU, OffsetV)>>::QUERIED_ATTRIBUTES
    }
}

#[allow(clippy::type_complexity)]
impl<'a> PassData<'a> for TileRenderPass {
    type Data = (
        Read<'a, Config>,
        Read<'a, DisplayConfig>,
        Read<'a, ActiveCamera>,
        ReadStorage<'a, Camera>,
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, AssetStorage<Texture>>,
        ReadStorage<'a, GlobalTransform>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, Tiles>,
        ReadTiles<'a, FlaggedSpriteRender>,
        ReadTiles<'a, Flipped>,
        ReadTiles<'a, Rgba>,
        ReadTiles<'a, GlobalTransform>,
    );
}

impl Pass for TileRenderPass {
    fn compile(&mut self, effect: NewEffect<'_>) -> Result<Effect, Error> {
        use std::mem;

        let mut builder = effect.simple(VERT_SRC, FRAG_SRC);
        builder
            .without_back_face_culling()
            .with_raw_constant_buffer(
                "ViewArgs",
                mem::size_of::<<ViewArgs as Uniform>::Std140>(),
                1,
            )
            .with_raw_vertex_buffer(Self::attributes(), SpriteInstance::size() as ElemStride, 1);
        setup_textures(&mut builder);
//        match self.transparency {
//            Some((mask, blend, depth)) => builder.with_blended_output("color", mask, blend, depth),
//            None => builder.with_output("color", Some(DepthMode::LessEqualWrite)),
//        };
        builder.with_output("color", None);

        self.map_transform = Some(GlobalTransform::default());

        builder.build()
    }

    fn apply<'a, 'b: 'a>(
        &'a mut self,
        encoder: &mut Encoder,
        effect: &mut Effect,
        mut factory: Factory,
        (
            game_settings,
            display_config,
            active,
            camera,
            sprite_sheet_storage,
            tex_storage,
            global,
            _local,
            tiles,
            tiles_sprites,
            tiles_flipped,
            tiles_rgba,
            tile_globals,
        ): <Self as PassData<'b>>::Data,
    ) {
        let camera_g = get_camera(active, &camera, &global);

        let (_, g) = camera_g.as_ref().unwrap();
        let global: amethyst::core::math::Matrix4<f32> = g.0;
        let camera_world_position = Vector3::new(global[12], global[13], global[14]);
        let camera_tile_position =
            tiles.world_to_tile(&camera_world_position.xyz(), &game_settings);

        // TODO: dont hardcode the tileset size multiplier, this should be stored in Tiles
        let view_tiles =
            display_config.dimensions.unwrap().0 as f32 / (16. * game_settings.graphics.scale); // Hardcoded for now, these should be out of the sprites and into the Tiles object

        let view_x = (camera_tile_position.x as f32 - view_tiles - 16.)
            .max(0.)
            .min(tiles.dimensions().x as f32) as u32;
        let view_y = (camera_tile_position.y as f32 - view_tiles - 16.)
            .max(0.)
            .min(tiles.dimensions().y as f32) as u32;

        let view_e_x = (camera_tile_position.x as f32 + view_tiles)
            .max(0.)
            .min(tiles.dimensions().x as f32) as u32;
        let view_e_y = (camera_tile_position.y as f32 + view_tiles)
            .max(0.)
            .min(tiles.dimensions().y as f32) as u32;

        // TODO: we should scale this to viewport from the camera
        for tile_id in tiles.iter_region(Vector4::new(view_x, view_y, view_e_x, view_e_y)) {
            let sprite_render = tiles_sprites.get(tile_id);
            if sprite_render.is_none() {
                continue;
            }
            let sprite_render = sprite_render.as_ref().unwrap();

            let flipped = tiles_flipped.get(tile_id).unwrap_or(&Flipped::None);
            let rgba = tiles_rgba.get(tile_id).unwrap_or(&Rgba::WHITE);

            let global = tile_globals.get(tile_id).unwrap();

            self.batch.add_sprite(
                sprite_render,
                Some(&global),
                Some(flipped),
                Some(rgba),
                &sprite_sheet_storage,
                &tex_storage,
            );
        }

        self.batch.encode(
            encoder,
            &mut factory,
            effect,
            camera_g,
            &sprite_sheet_storage,
            &tex_storage,
        );
        self.batch.reset();
    }
}

#[derive(Clone, Debug)]
struct TextureDrawData {
    texture_handle: Handle<Texture>,
    render: FlaggedSpriteRender,
    flipped: Option<Flipped>,
    rgba: Option<Rgba>,
    transform: GlobalTransform,
}

#[derive(Clone, Default, Debug)]
struct TextureBatch {
    textures: Vec<TextureDrawData>,
}

impl TextureBatch {
    pub fn add_sprite(
        &mut self,
        sprite_render: &FlaggedSpriteRender,
        global: Option<&GlobalTransform>,
        flipped: Option<&Flipped>,
        rgba: Option<&Rgba>,
        sprite_sheet_storage: &AssetStorage<SpriteSheet>,
        tex_storage: &AssetStorage<Texture>,
    ) {
        if let Some(global) = global {
            let texture_handle = match sprite_sheet_storage.get(&sprite_render.handle) {
                Some(sprite_sheet) => {
                    if tex_storage.get(&sprite_sheet.texture).is_none() {
                        warn!(
                            "Texture not loaded for texture: `{:?}`.",
                            sprite_sheet.texture
                        );
                        return;
                    }

                    sprite_sheet.texture.clone()
                }
                None => {
                    warn!(
                        "Sprite sheet not loaded for sprite_render: `{:?}`.",
                        sprite_render
                    );
                    return;
                }
            };

            self.textures.push(TextureDrawData {
                texture_handle,
                render: sprite_render.clone(),
                flipped: flipped.cloned(),
                rgba: rgba.cloned(),
                transform: *global,
            });
        }
    }

    pub fn encode(
        &self,
        encoder: &mut Encoder,
        factory: &mut Factory,
        effect: &mut Effect,
        camera: Option<(&Camera, &GlobalTransform)>,
        sprite_sheet_storage: &AssetStorage<SpriteSheet>,
        tex_storage: &AssetStorage<Texture>,
    ) {
        use gfx::{
            buffer,
            memory::{Bind, Typed},
            Factory,
        };

        if self.textures.is_empty() {
            return;
        }

        // Sprite vertex shader
        set_view_args(effect, encoder, camera);

        // We might be able to improve performance here if we
        // preallocate the maximum needed capacity. We need to
        // iterate over the sprites though to find out the longest
        // chain of sprites with the same texture, so we would need
        // to check if it actually results in an improvement over just
        // doing the allocations.
        let mut num_instances = 0;
        let num_quads = self.textures.len();
        let mut instance_data: Vec<f32> = Vec::with_capacity(num_quads * 15);

        for (i, quad) in self.textures.iter().enumerate() {
            // Get actual texture to use.
            let texture = tex_storage
                .get(&quad.texture_handle)
                .expect("Unable to get texture of sprite");

            let sprite_sheet = sprite_sheet_storage.get(&quad.render.handle).expect(
                "Unreachable: Existence of sprite sheet checked when collecting the sprites",
            );

            // Append sprite to instance data.
            let sprite_data = &sprite_sheet.sprites[quad.render.sprite_number];

            let transform = &quad.transform.0;

            let dir_x = transform.column(0) * sprite_data.width;
            let dir_y = transform.column(1) * sprite_data.height;

            // The offsets are negated to shift the sprite left and down relative to the entity, in
            // regards to pivot points. This is the convention adopted in:
            //
            // * libgdx: <https://gamedev.stackexchange.com/q/22553>
            // * godot: <https://godotengine.org/qa/9784>
            let pos = transform
                * Vector4::new(-sprite_data.offsets[0], -sprite_data.offsets[1], 0.0, 1.0);

            let rgba = quad.rgba.unwrap_or(Rgba::WHITE);

            instance_data.extend(&[
                dir_x.x,
                dir_x.y,
                dir_y.x,
                dir_y.y,
                pos.x,
                pos.y,
                sprite_data.tex_coords.left,
                sprite_data.tex_coords.right,
                sprite_data.tex_coords.bottom,
                sprite_data.tex_coords.top,
            ]);
            num_instances += 1;

            // Need to flush outstanding draw calls due to state switch (texture).
            //
            // 1. We are at the last sprite and want to submit all pending work.
            // 2. The next sprite will use a different texture triggering a flush.
            let need_flush = i >= num_quads - 1
                || self.textures[i + 1].texture_handle.id() != quad.texture_handle.id();

            if need_flush {
                add_texture(effect, texture);

                let vbuf = factory
                    .create_buffer_immutable(&instance_data, buffer::Role::Vertex, Bind::empty())
                    .expect("Unable to create immutable buffer for `TextureBatch`");

                for _ in TileRenderPass::attributes() {
                    effect.data.vertex_bufs.push(vbuf.raw().clone());
                }

                effect.draw(
                    &Slice {
                        start: 0,
                        end: 6,
                        base_vertex: 0,
                        instances: Some((num_instances, 0)),
                        buffer: Default::default(),
                    },
                    encoder,
                );

                effect.clear();

                num_instances = 0;
                instance_data.clear();
            }
        }
    }

    pub fn reset(&mut self) {
        self.textures.clear();
    }
}
