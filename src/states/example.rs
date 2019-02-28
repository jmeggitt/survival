use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::{Entity, SystemData},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
    utils::application_root_dir
};


use specs_static::WorldExt;

use crate::tiles::{Tiles, TileId, WriteTiles};
use crate::Player;

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

// Initialize a sprite as a reference point at a fixed location
fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(100.0);
    transform.set_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .build()
}

fn init_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -1000.0, 1000.0, -1000.0, 1000.0,
        )))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
}

pub struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let circle_sprite_sheet_handle =
            load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
        //let background_sprite_sheet_handle =
        //    load_sprite_sheet(world, "Background.png", "Background.ron");

        // let _background = init_background_sprite(world, &background_sprite_sheet_handle);
        let _reference = init_reference_sprite(world, &circle_sprite_sheet_handle);
        let parent = init_player(world, &circle_sprite_sheet_handle);
        init_camera(world, parent);

        let map_sprite_sheet_handle = load_sprite_sheet(world, "spritesheets/Bisasam_24x24.png", "spritesheets/Bisasam_24x24.ron");

        world.register_tile_comp::<amethyst::renderer::SpriteRender, TileId>();
        world.register_tile_comp::<amethyst::renderer::Flipped, TileId>();
        world.register_tile_comp::<amethyst::renderer::Rgba, TileId>();

        world.add_resource(Tiles::new(12, 12));

        {
            let tiles = world.read_resource::<Tiles>();
            let mut sprites: WriteTiles<SpriteRender> = SystemData::fetch(&world.res);
            for tile_id in tiles.iter_all() {
                sprites.insert(tile_id, SpriteRender {
                    sprite_sheet: map_sprite_sheet_handle.clone(),
                    sprite_number: 4,
                });
            }
        }

        let game_config = crate::settings::GameSettings::load(application_root_dir().unwrap().join("resources").join("game_settings.ron"));
        world.add_resource(game_config);

    }
}