//! Flat forward drawing pass that mimics a blit.

use derivative::Derivative;
use gfx::pso::buffer::ElemStride;
use gfx_core::state::{Blend, ColorMask};
use glsl_layout::Uniform;

use amethyst::assets::{AssetStorage, Handle};
use amethyst::core::{
    nalgebra::Vector4,
    specs::prelude::{Read, ReadStorage, ReadExpect},
    transform::{Transform, GlobalTransform},
};
use amethyst::error::Error;

use amethyst::renderer::{
    ActiveCamera, Camera,
    get_camera,
    pipe::{
        pass::{Pass, PassData},
        DepthMode, Effect, NewEffect,
    },
    Flipped, SpriteSheet,
    Texture, TextureHandle,
    Encoder, Factory,
    Attributes, Query, VertexFormat,
    DisplayConfig,
    Color, Rgba,
    Resources,
};

use crate::settings::GameSettings;
use crate::components::FlaggedSpriteRender;

use super::*;
use super::util::{setup_textures, ViewArgs, set_view_args, add_texture, default_transparency,};

use crate::tiles::*;

type Slice = gfx::Slice<Resources>;

/// Draws sprites on a 2D quad.
#[derive(Derivative, Clone, Debug)]
#[derivative(Default(bound = "Self: Pass"))]
pub struct DrawFlat2D {
    #[derivative(Default(value = "default_transparency()"))]
    transparency: Option<(ColorMask, Blend, Option<DepthMode>)>,
    batch: TextureBatch,
    map_transform: Option<GlobalTransform>,
}

impl DrawFlat2D
where
    Self: Pass,
{
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
        <SpriteInstance as Query<(DirX, DirY, Pos, OffsetU, OffsetV, Depth, Color)>>::QUERIED_ATTRIBUTES
    }
}

#[allow(clippy::type_complexity)]
impl<'a> PassData<'a> for DrawFlat2D {
    type Data = (
        Read<'a, GameSettings>,
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

    );
}

impl Pass for DrawFlat2D {
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
        setup_textures(&mut builder, &TEXTURES);
        match self.transparency {
            Some((mask, blend, depth)) => builder.with_blended_output("color", mask, blend, depth),
            None => builder.with_output("color", Some(DepthMode::LessEqualWrite)),
        };

        self.map_transform = Some(GlobalTransform::default());

        builder.build()
    }

    #[allow(clippy::extra_unused_lifetimes)]
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
        ): <Self as PassData<'a>>::Data,
    ) {
        let camera_g = get_camera(active, &camera, &global);

        let camera_position;
        {
            let matrix = (camera_g.as_ref().unwrap().1).0;
            camera_position = nalgebra::Vector2::new(*(matrix.get(12).unwrap()), *(matrix.get(13).unwrap()));
        }
        // Calculate the scale of how much we can view...from...what?
        // this should be resolution / (tile width * scale(
        // TODO: dont hardcode the tileset size multiplier, this should be stored in Tiles
        let view_tiles = display_config.dimensions.unwrap().0 as f32 / (20. * game_settings.graphics.scale); // Hardcoded for now, these should be out of the sprites and into the Tiles object
        let camera_tile_x = (camera_position.x / 20. / game_settings.graphics.scale);
        let camera_tile_y = (camera_position.y / 20. / game_settings.graphics.scale).abs();

        let view_x = (camera_tile_x - view_tiles - 20.).max(0.).min(tiles.dimensions().x as f32) as u32;
        let view_y = (camera_tile_y - view_tiles - 20.).max(0.).min(tiles.dimensions().y as f32) as u32;

        let view_e_x = (camera_tile_x + view_tiles).max(0.).min(tiles.dimensions().x as f32) as u32;
        let view_e_y = (camera_tile_y + view_tiles).max(0.).min(tiles.dimensions().y as f32) as u32;

        println!("Viewing: real={:?}, camera=({}, {}), {}, {}, {}, {}", camera_position, camera_tile_x, camera_tile_y, view_x, view_y, view_e_x, view_e_y);

        // TODO: we should scale this to viewport from teh camera
        for tile_id in tiles.iter_region(
            Vector4::new(
                view_x,
                view_y,
                view_e_x,
                view_e_y)
        ) {
            let sprite_render = tiles_sprites.get(tile_id);
            if sprite_render.is_none() {
                continue;
            }
            let sprite_render = sprite_render.as_ref().unwrap();

            let flipped = tiles_flipped.get(tile_id).unwrap_or(&Flipped::None);
            let rgba = tiles_rgba.get(tile_id).unwrap_or(&Rgba::WHITE);

            let sprite_sheet = sprite_sheet_storage
                .get(&sprite_render.sprite_sheet);
            if sprite_sheet.is_none() {
                continue;
            }
            let sprite_sheet = sprite_sheet.as_ref().unwrap();

            let coords = tile_id.coords(tiles.dimensions());
            let mut transform = Transform::default();

            let width = sprite_sheet.sprites[sprite_render.sprite_number].width;
            let height = sprite_sheet.sprites[sprite_render.sprite_number].height;
            transform.set_xyz((coords.0 * width * game_settings.graphics.scale),
                              -1. * (coords.1 * height * game_settings.graphics.scale),
                              0.);
            transform.set_scale(game_settings.graphics.scale, game_settings.graphics.scale, 0.);

            //println!("Setting at: {}, {}: coord={},width={},scale={}", (coords.0 * width * game_settings.graphics.scale),
            //         (coords.1 * height * game_settings.graphics.scale), coords.1, width, game_settings.graphics.scale);

            let mut global = GlobalTransform::default();
            global.0 = transform.matrix();

            self.batch.add_sprite(
                sprite_render,
                Some(&global),
                Some(flipped),
                Some(rgba),
                &sprite_sheet_storage,
                &tex_storage,
            );
            //self.batch.sort();
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
enum TextureDrawData {
    Sprite {
        texture_handle: Handle<Texture>,
        render: FlaggedSpriteRender,
        flipped: Option<Flipped>,
        rgba: Option<Rgba>,
        transform: GlobalTransform,
    },
    Image {
        texture_handle: Handle<Texture>,
        transform: GlobalTransform,
        flipped: Option<Flipped>,
        rgba: Option<Rgba>,
        width: usize,
        height: usize,
    },
}

impl TextureDrawData {
    pub fn texture_handle(&self) -> &Handle<Texture> {
        match self {
            TextureDrawData::Sprite { texture_handle, .. } => texture_handle,
            TextureDrawData::Image { texture_handle, .. } => texture_handle,
        }
    }

    pub fn tex_id(&self) -> u32 {
        match self {
            TextureDrawData::Sprite { texture_handle, .. } => texture_handle.id(),
            TextureDrawData::Image { texture_handle, .. } => texture_handle.id(),
        }
    }

    pub fn flipped(&self) -> &Option<Flipped> {
        match self {
            TextureDrawData::Sprite { flipped, .. } => flipped,
            TextureDrawData::Image { flipped, .. } => flipped,
        }
    }
}

#[derive(Clone, Default, Debug)]
struct TextureBatch {
    textures: Vec<TextureDrawData>,
}

impl TextureBatch {
    pub fn add_image(
        &mut self,
        texture_handle: &TextureHandle,
        global: Option<&GlobalTransform>,
        flipped: Option<&Flipped>,
        rgba: Option<&Rgba>,
        tex_storage: &AssetStorage<Texture>,
    ) {
        let global = match global {
            Some(v) => v,
            None => return,
        };

        let texture_dims = match tex_storage.get(&texture_handle) {
            Some(tex) => tex.size(),
            None => {
                // TODO: Slog
                //warn!("Texture not loaded for texture: `{:?}`.", texture_handle);
                return;
            }
        };

        self.textures.push(TextureDrawData::Image {
            texture_handle: texture_handle.clone(),
            transform: *global,
            flipped: flipped.cloned(),
            rgba: rgba.cloned(),
            width: texture_dims.0,
            height: texture_dims.1,
        });
    }

    pub fn add_sprite(
        &mut self,
        sprite_render: &FlaggedSpriteRender,
        global: Option<&GlobalTransform>,
        flipped: Option<&Flipped>,
        rgba: Option<&Rgba>,
        sprite_sheet_storage: &AssetStorage<SpriteSheet>,
        tex_storage: &AssetStorage<Texture>,
    ) {
        let global = match global {
            Some(v) => v,
            None => return,
        };

        let texture_handle = match sprite_sheet_storage.get(&sprite_render.sprite_sheet) {
            Some(sprite_sheet) => {
                if tex_storage.get(&sprite_sheet.texture).is_none() {
                    // TODO: Slog
                    //warn!(
                    //    "Texture not loaded for texture: `{:?}`.",
                    //    sprite_sheet.texture
                    //);
                    return;
                }

                sprite_sheet.texture.clone()
            }
            None => {
                // TODO: Slog
                //warn!(
                //    "Sprite sheet not loaded for sprite_render: `{:?}`.",
                //    sprite_render
                //);
                return;
            }
        };

        self.textures.push(TextureDrawData::Sprite {
            texture_handle,
            render: sprite_render.clone(),
            flipped: flipped.cloned(),
            rgba: rgba.cloned(),
            transform: *global,
        });
    }

    /// Optimize the sprite order to generating more coherent batches.
    pub fn sort(&mut self) {
        // Only takes the texture into account for now.
        self.textures.sort_by(|a, b| a.tex_id().cmp(&b.tex_id()));
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
        let mut instance_data = Vec::<f32>::new();
        let mut num_instances = 0;
        let num_quads = self.textures.len();

        for (i, quad) in self.textures.iter().enumerate() {
            let texture = tex_storage
                .get(&quad.texture_handle())
                .expect("Unable to get texture of sprite");

            let (flip_horizontal, flip_vertical) = match quad.flipped() {
                Some(Flipped::Horizontal) => (true, false),
                Some(Flipped::Vertical) => (false, true),
                Some(Flipped::Both) => (true, true),
                _ => (false, false),
            };

            let (dir_x, dir_y, pos, uv_left, uv_right, uv_top, uv_bottom, rgba) = match quad {
                TextureDrawData::Sprite {
                    render,
                    transform,
                    rgba,
                    ..
                } => {
                    let sprite_sheet = sprite_sheet_storage
                        .get(&render.sprite_sheet)
                        .expect(
                            "Unreachable: Existence of sprite sheet checked when collecting the sprites",
                        );

                    // Append sprite to instance data.
                    let sprite_data = &sprite_sheet.sprites[render.sprite_number];

                    let tex_coords = &sprite_data.tex_coords;
                    let (uv_left, uv_right) = if flip_horizontal {
                        (tex_coords.right, tex_coords.left)
                    } else {
                        (tex_coords.left, tex_coords.right)
                    };
                    let (uv_bottom, uv_top) = if flip_vertical {
                        (tex_coords.top, tex_coords.bottom)
                    } else {
                        (tex_coords.bottom, tex_coords.top)
                    };

                    let transform = &transform.0;

                    let dir_x = transform.column(0) * sprite_data.width;
                    let dir_y = transform.column(1) * sprite_data.height;

                    // The offsets are negated to shift the sprite left and down relative to the entity, in
                    // regards to pivot points. This is the convention adopted in:
                    //
                    // * libgdx: <https://gamedev.stackexchange.com/q/22553>
                    // * godot: <https://godotengine.org/qa/9784>
                    let pos = transform
                        * Vector4::new(-sprite_data.offsets[0], -sprite_data.offsets[1], 0.0, 1.0);

                    (
                        dir_x, dir_y, pos, uv_left, uv_right, uv_top, uv_bottom, rgba,
                    )
                }
                TextureDrawData::Image {
                    transform,
                    width,
                    height,
                    rgba,
                    ..
                } => {
                    let (uv_left, uv_right) = if flip_horizontal {
                        (1.0, 0.0)
                    } else {
                        (0.0, 1.0)
                    };
                    let (uv_bottom, uv_top) = if flip_vertical {
                        (1.0, 0.0)
                    } else {
                        (0.0, 1.0)
                    };

                    let transform = &transform.0;

                    let dir_x = transform.column(0) * (*width as f32);
                    let dir_y = transform.column(1) * (*height as f32);

                    let pos = transform * Vector4::new(1.0, 1.0, 0.0, 1.0);

                    (
                        dir_x, dir_y, pos, uv_left, uv_right, uv_top, uv_bottom, rgba,
                    )
                }
            };
            let rgba = rgba.unwrap_or(Rgba::WHITE);
            instance_data.extend(&[
                dir_x.x, dir_x.y, dir_y.x, dir_y.y, pos.x, pos.y, uv_left, uv_right, uv_bottom,
                uv_top, pos.z, rgba.0, rgba.1, rgba.2, rgba.3,
            ]);
            num_instances += 1;

            // Need to flush outstanding draw calls due to state switch (texture).
            //
            // 1. We are at the last sprite and want to submit all pending work.
            // 2. The next sprite will use a different texture triggering a flush.
            let need_flush = i >= num_quads - 1
                || self.textures[i + 1].texture_handle().id() != quad.texture_handle().id();

            if need_flush {
                add_texture(effect, texture);

                let vbuf = factory
                    .create_buffer_immutable(&instance_data, buffer::Role::Vertex, Bind::empty())
                    .expect("Unable to create immutable buffer for `TextureBatch`");

                for _ in DrawFlat2D::attributes() {
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
