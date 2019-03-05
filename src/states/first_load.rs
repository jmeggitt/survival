use amethyst::{
    ecs::{World},
    renderer::{Texture, TextureMetadata, PngFormat, SpriteSheetFormat, SpriteSheet, SpriteSheetHandle},
    assets::{AssetStorage, Loader},
    StateEvent, Trans, StateData,
    assets::ProgressCounter,
};
use specs_static::WorldExt;

use slog::slog_trace;

use crate::SurvivalData;
use crate::settings;

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

pub struct State {
    progress_counter: ProgressCounter,
    log: slog::Logger,
}
impl State {
    pub fn new(root_logger: slog::Logger) -> Self {
        Self {
            progress_counter: ProgressCounter::default(),
            log: root_logger,
        }
    }
}
impl<'a, 'b> amethyst::State<SurvivalData<'a, 'b>, StateEvent> for State {
    fn on_start(&mut self, data: StateData<'_, SurvivalData<'_, '_>>) {
        let world = data.world;

        slog_trace!(self.log, "Changed state to first_load");

        // Load sprite sheets
        let _default_sprite_sheet = load_sprite_sheet(world, "spritesheets/Bisasam_24x24.png", "spritesheets/Bisasam_24x24.ron");
        // How do we pass this along?
        world.res.fetch_mut::<settings::Context>().spritesheet = Some(_default_sprite_sheet);

            // Load items
        world.add_resource(AssetStorage::<crate::assets::ItemStorage>::default());
        {
            let loader = &world.read_resource::<Loader>();
            let _handle = loader.load(
                "resources/data/items.ron",
                amethyst::assets::RonFormat,
                (),
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<crate::assets::ItemStorage>>(),
            );
        }

        // Register tile components
        world.register_tile_comp::<crate::components::FlaggedSpriteRender, crate::tiles::TileId>();
        world.register_tile_comp::<amethyst::renderer::Flipped, crate::tiles::TileId>();
        world.register_tile_comp::<amethyst::renderer::Rgba, crate::tiles::TileId>();
        world.register_tile_comp::<amethyst::core::transform::GlobalTransform, crate::tiles::TileId>();
        world.register_tile_comp::<crate::tiles::TileEntities, crate::tiles::TileId>();
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, SurvivalData<'_, '_>>,
        _: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        slog_trace!(self.log, "Event First Load");
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        Trans::Switch(Box::new(super::Level::new(self.log.clone())))
    }
}