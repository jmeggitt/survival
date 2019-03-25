use amethyst::{
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::{Entity, SystemData},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
    utils::application_root_dir,
};

use crate::components::{Player, TilePosition};
use crate::tiles::{Tiles, WriteTiles};

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
fn init_reference_sprite(
    world: &mut World,
    sprite_sheet: &SpriteSheetHandle,
    tiles: crate::tiles::Tiles,
    game_settings: &crate::settings::Config,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(100.0);
    transform.set_translation_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(TilePosition::from_transform(
            &transform,
            tiles,
            game_settings,
        ))
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .build()
}

fn init_player(
    world: &mut World,
    sprite_sheet: &SpriteSheetHandle,
    tiles: crate::tiles::Tiles,
    game_settings: &crate::settings::Config,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(50.0);
    transform.set_translation_y(50.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(TilePosition::from_transform(
            &transform,
            tiles,
            game_settings,
        ))
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(crate::components::TimeAvailable::default())
        .with(Transparent)
        .build()
}

fn init_camera(
    world: &mut World,
    parent: Entity,
    tiles: crate::tiles::Tiles,
    game_settings: &crate::settings::Config,
) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(TilePosition::from_transform(
            &transform,
            tiles,
            game_settings,
        ))
        .with(Camera::from(Projection::orthographic(
            -1000.0, 1000.0, -1000.0, 1000.0,
        )))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
}

pub struct Example;

impl SimpleState for Example {
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        amethyst_imgui::handle_imgui_events(data.world, &event);

        if let StateEvent::Window(event) = &event {
            if amethyst::input::is_close_requested(&event) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit).
    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //let world = &data.world;
        // Test imgui draw calls
        //let mut imgui: Write<EventChannel<crate::systems::ui::ImguiDraw>> = SystemData::fetch(&world.res);
        //imgui.single_write(std::sync::Arc::new(|ui: &amethyst_imgui::imgui::Ui| {
        //    ui.show_demo_window(&mut true);
        // }));
        Trans::None
    }

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //world.register_tile_comp::<crate::components::FlaggedSpriteRender, TileId>();
        //world.register_tile_comp::<amethyst::renderer::Flipped, TileId>();
        //world.register_tile_comp::<amethyst::renderer::Rgba, TileId>();
        //world.register_tile_comp::<amethyst::core::transform::GlobalTransform, TileId>();
        //world.register_tile_comp::<TileEntities, TileId>();

        let tiles = Tiles::new(100, 100);
        let game_settings = crate::settings::Config::load(
            application_root_dir()
                .unwrap()
                .join("resources")
                .join("game_settings.ron"),
        );

        // Init the test entities and map
        {
            let circle_sprite_sheet_handle =
                load_sprite_sheet(world, "Circle_Spritesheet.png", "Circle_Spritesheet.ron");
            //let background_sprite_sheet_handle =
            //    load_sprite_sheet(world, "Background.png", "Background.ron");

            // let _background = init_background_sprite(world, &background_sprite_sheet_handle);
            let _reference =
                init_reference_sprite(world, &circle_sprite_sheet_handle, tiles, &game_settings);
            let parent = init_player(world, &circle_sprite_sheet_handle, tiles, &game_settings);
            init_camera(world, parent, tiles, &game_settings);

            let map_sprite_sheet_handle = load_sprite_sheet(
                world,
                "spritesheets/Bisasam_16x16.png",
                "spritesheets/Bisasam_16x16.ron",
            );

            {
                let mut sprites: WriteTiles<crate::components::FlaggedSpriteRender> =
                    SystemData::fetch(&world.res);
                let mut transforms: WriteTiles<amethyst::core::transform::GlobalTransform> =
                    SystemData::fetch(&world.res);
                let mut tile_entities_map: WriteTiles<crate::tiles::TileEntities> =
                    SystemData::fetch(&world.res);
                for tile_id in tiles.iter_all() {
                    tile_entities_map.insert(tile_id, crate::tiles::TileEntities::default());

                    sprites.insert(
                        tile_id,
                        crate::components::FlaggedSpriteRender {
                            sprite_sheet: map_sprite_sheet_handle.clone(),
                            sprite_number: 3,
                        },
                    );

                    let coords = tile_id.coords(tiles.dimensions());
                    let mut transform = Transform::default();

                    let width = 20.;
                    let height = 20.;
                    transform.set_translation_xyz(
                        coords.0 * width * game_settings.graphics.scale,
                        -1. * (coords.1 * height * game_settings.graphics.scale),
                        0.,
                    );
                    transform.set_scale(
                        game_settings.graphics.scale,
                        game_settings.graphics.scale,
                        0.,
                    );

                    //println!("Setting at: {}, {}: coord={},width={},scale={}", (coords.0 * width * game_settings.graphics.scale),
                    //         (coords.1 * height * game_settings.graphics.scale), coords.1, width, game_settings.graphics.scale);

                    let mut global = amethyst::core::transform::GlobalTransform::default();
                    global.0 = transform.matrix();
                    transforms.insert(tile_id, global);
                }
            }
        }
        //let display_config = amethyst::renderer::DisplayConfig::load(application_root_dir().unwrap().join("resources").join("display_config.ron"));
        //world.add_resource(display_config);

        //world.add_resource(crate::settings::Context { logs: crate::settings::Logs { root: self.log.clone() } });
        world.add_resource(tiles);
        //world.add_resource(game_settings);

        // Load items
        /*
        world.add_resource(AssetStorage::<crate::assets::item::ItemStorage>::default());
        {
            let loader = &world.read_resource::<Loader>();
            let _handle = loader.load(
                "resources/data/items.ron",
                amethyst::assets::RonFormat,
                (),
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<crate::assets::item::ItemStorage>>(),
            );
        }*/

        // Initialize the UI
        world.exec(|mut creator: amethyst::ui::UiCreator<'_>| {
            let entity = creator.create("ui/main_ui.ron", ());
            println!("Created ui: {}", entity.id());
        });
    }
}
