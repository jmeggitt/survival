use amethyst::{
    assets::{AssetStorage},
    renderer::{Projection, Camera, SpriteSheet, SpriteRender, SpriteSheetHandle, Transparent},
    core::{
        Parent,
        components::{Transform, GlobalTransform}
    },
    ecs::{Entity, EntityBuilder, Builder, SystemData, World},
    StateEvent, Trans, StateData,
    assets::ProgressCounter,
};

use slog::slog_trace;

use crate::SurvivalData;
use crate::tiles::{Tiles, TileId, WriteTiles};
use crate::components::{TimeAvailable, FlaggedSpriteRender, Player, TilePosition};
use crate::tiles::TileEntities;
use crate::settings;

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle, tiles: Tiles, game_settings: &settings::Config) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(50.0);
    transform.set_y(50.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(TilePosition::from_transform(&transform, tiles, game_settings))
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(TimeAvailable::default())
        .with(Transparent)
        .build()
}

fn init_camera(world: &mut World, parent: Entity, tiles: Tiles, game_settings: &settings::Config) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(TilePosition::from_transform(&transform, tiles, game_settings))
        .with(Camera::from(Projection::orthographic(
            -1000.0, 1000.0, -1000.0, 1000.0,
        )))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
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
        slog_trace!(self.log, "Changed state to Level");

        // Load the level
        let tiles = Tiles::new(100, 100);
        {
            let context = world.res.fetch::<settings::Context>().clone();
            let map_sprite_sheet_handle = context.spritesheet.as_ref().unwrap();
            let mut game_settings = world.res.fetch::<settings::Config>().clone();

            let player = init_player(world, map_sprite_sheet_handle, tiles, &game_settings);
            let _camera = init_camera(world, player, tiles, &game_settings);

            let mut sprites: WriteTiles<FlaggedSpriteRender> = SystemData::fetch(&world.res);
            let mut transforms: WriteTiles<GlobalTransform> = SystemData::fetch(&world.res);
            let mut tile_entities_map: WriteTiles<TileEntities> = SystemData::fetch(&world.res);
            for tile_id in tiles.iter_all() {
                tile_entities_map.insert_default(tile_id);

                sprites.insert(tile_id, FlaggedSpriteRender {
                    sprite_sheet: map_sprite_sheet_handle.clone(),
                    sprite_number: 3,
                });

                let coords = tile_id.coords(tiles.dimensions());
                let mut transform = Transform::default();

                let width = 20.;
                let height = 20.;
                transform.set_xyz(coords.0 * width * game_settings.graphics.scale,
                                  -1. * (coords.1 * height * game_settings.graphics.scale),
                                  0.);
                transform.set_scale(game_settings.graphics.scale, game_settings.graphics.scale, 0.);

                let mut global = GlobalTransform::default();
                global.0 = transform.matrix();
                transforms.insert(tile_id, global);
            }
        }

        world.add_resource(tiles);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, SurvivalData<'_, '_>>,
        _: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        slog_trace!(self.log, "Event Level");
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        // Just swap straight to Paused
        Trans::Push(Box::new(super::Paused::new(self.log.clone())))
    }
}