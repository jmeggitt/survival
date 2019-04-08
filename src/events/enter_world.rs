use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use amethyst::renderer::SpriteSheet;
use amethyst::{
    assets::AssetStorage,
    assets::Progress,
    assets::ProgressCounter,
    assets::Tracker,
    core::components::{GlobalTransform, Transform},
    ecs::{Builder, Entity, Read, SystemData, World, WriteExpect, WriteStorage},
    renderer::{Camera, Projection, Rgba, SpriteRender, SpriteSheetHandle, Transparent},
    StateData, StateEvent, Trans,
};
use log::error;
use log::info;
use shred::ReadExpect;

use crate::components::{Actionable, FlaggedSpriteRender, Player, TimeAvailable};
use crate::settings;
use crate::tiles::TileEntities;
use crate::tiles::{TileAsset, TileAssets};
use crate::tiles::{Tiles, WriteTiles};
use crate::GameDispatchers;

fn init_player(
    world: &mut World,
    sprite_sheet: &SpriteSheetHandle,
    tiles: Tiles,
    game_settings: &settings::Config,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(300.0, -300.0, 0.0);
    transform.set_scale(
        game_settings.graphics.scale,
        game_settings.graphics.scale,
        1.,
    );

    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 25,
        })
        .with(TimeAvailable::default())
        .with(Actionable::default())
        .with(Transparent)
        .with(Rgba::RED)
        .build()
}

fn init_camera(world: &mut World, _: Entity, tiles: Tiles, game_settings: &settings::Config) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -1000.0, 1000.0, -1000.0, 1000.0,
        )))
        .with(transform)
        .build();
}

pub struct Level;

impl<'a, 'b> amethyst::State<GameDispatchers<'a, 'b>, StateEvent> for Level {
    fn on_start(&mut self, data: StateData<'_, GameDispatchers<'_, '_>>) {
        let world = data.world;
        info!("Started level setup");

        // Load the level
        let tiles = Tiles::new(100, 100);
        let context = world.res.fetch::<settings::Context>().clone();
        let map_sprite_sheet_handle = context.as_ref().unwrap();
        let game_settings = world.res.fetch::<settings::Config>().clone();

        let player = init_player(world, map_sprite_sheet_handle, tiles, &game_settings);
        init_camera(world, player, tiles, &game_settings);

        world.add_resource(tiles);
        info!("Finished level setup");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameDispatchers<'_, '_>>,
        event: StateEvent,
    ) -> Trans<GameDispatchers<'a, 'b>, StateEvent> {
        amethyst_imgui::handle_imgui_events(data.world, &event);
        Trans::None
    }

    fn update(
        &mut self,
        state: StateData<'_, GameDispatchers<'_, '_>>,
    ) -> Trans<GameDispatchers<'a, 'b>, StateEvent> {
        if !unsafe { SHEET_INIT.load(Ordering::SeqCst) } {
            let context = state.world.res.fetch::<settings::Context>().clone();
            let map_sprite_sheet_handle = context.as_ref().unwrap();

            let sheet = state.world.read_resource::<AssetStorage<SpriteSheet>>();
            match sheet.get(map_sprite_sheet_handle) {
                Some(sheet) => {
                    let mut tiles = state.world.write_resource::<TileAssets>();
                    for sprite in sheet.sprites.iter() {
                        tiles.0.push(TileAsset {
                            texture: sheet.texture.clone(),
                            sprite: sprite.clone(),
                            tint: Rgba::WHITE,
                        });
                    }

                    unsafe {
                        SHEET_INIT.store(true, Ordering::SeqCst);
                    }
                }
                None => (),
            }
        }

        state.data.update(state.world);
        Trans::None
    }
}

pub static mut SHEET_INIT: AtomicBool = AtomicBool::new(false);
