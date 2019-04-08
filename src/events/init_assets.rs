use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    ecs::Component,
    ecs::World,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
    StateData, StateEvent, Trans,
};
use log::info;
use log::trace;

use crate::events::Level;
use crate::settings;
use crate::specs_static::WorldExt;
use crate::GameDispatchers;
use amethyst::assets::Progress;
use crate::tiles::TileAssets;

fn load_sprite_sheet(
    world: &mut World,
    png_path: &str,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = loader.load(
        png_path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    );

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        progress_counter,
        &sprite_sheet_store,
    )
}

#[derive(Default)]
pub struct FirstLoad {
//    progress_counter: ProgressCounter,
}

impl<'a, 'b> amethyst::State<GameDispatchers<'a, 'b>, StateEvent> for FirstLoad {
    fn on_start(&mut self, data: StateData<'_, GameDispatchers<'_, '_>>) {
        let world = data.world;

        trace!("Changed state to first_load");

        let mut progress = ProgressCounter::new();

        let default_sprite_sheet = load_sprite_sheet(
            world,
            "spritesheets/tileset_16x16.png",
            "spritesheets/tileset_16x16.ron",
            &mut progress,
        );

        // How do we pass this along?
        *world.res.fetch_mut::<settings::Context>() = Some(default_sprite_sheet);

        crate::assets::StorageSource::<crate::assets::Item>::apply(
            &std::path::Path::new("resources/data/items.ron"),
            world,
        )
        .unwrap();

        // Register tile components
        world.register_tile_comp::<crate::components::FlaggedSpriteRender, crate::tiles::TileId>();
        world.register_tile_comp::<amethyst::renderer::Flipped, crate::tiles::TileId>();
        world.register_tile_comp::<amethyst::renderer::Rgba, crate::tiles::TileId>();
        world
            .register_tile_comp::<amethyst::core::transform::GlobalTransform, crate::tiles::TileId>(
            );
        world.register_tile_comp::<crate::tiles::TileEntities, crate::tiles::TileId>();
        world.register_tile_comp::<crate::render::ChunkRender, (i32, i32)>();
        //        world.register_tile_comp::<crate::render::ChunkRender, (i32, i32)>();
        //        world.register::<crate::tiles::TileAssets>();
//                *world.res.fetch_mut::<crate::tiles::TileAssets>() = TileAssets(Vec::new());
        world.add_resource(TileAssets(Vec::new()));
        world.add_resource(progress);

        world.register_tile_comp::<crate::components::Impassable, crate::tiles::TileId>();
        info!("Finished initial asset load");
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameDispatchers<'_, '_>>,
        _: StateEvent,
    ) -> Trans<GameDispatchers<'a, 'b>, StateEvent> {
        trace!("Event First Load");
        Trans::None
    }

    fn update(&mut self, _: StateData<'_, GameDispatchers<'_, '_>>)
        -> Trans<GameDispatchers<'a, 'b>, StateEvent> {
//        if self.progress_counter.is_complete() {
//            info!("Completed asset load");
            Trans::Switch(Box::new(Level))
//        } else {
//            Trans::None
//        }
    }
}
