use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use amethyst::renderer::SpriteSheet;
use amethyst::{
    assets::AssetStorage,
    core::components::Transform,
    ecs::Builder,
    renderer::{Camera, Projection, Rgba, SpriteRender, Transparent},
    StateData, StateEvent, Trans,
};
use log::info;

use crate::components::{Actionable, Player, TimeAvailable};
use crate::settings;
use crate::tiles::{TileAsset, TileAssets, Tiles};
use crate::GameDispatchers;

pub struct Level;

impl<'a, 'b> amethyst::State<GameDispatchers<'a, 'b>, StateEvent> for Level {
    fn on_start(&mut self, data: StateData<'_, GameDispatchers<'_, '_>>) {
        let world = data.world;
        info!("Started level setup");

        // Fetch resources needed for init
        world.add_resource(Tiles::new(100, 100));
        let context = world.res.fetch::<settings::Context>().clone();
        let map_sprite_sheet_handle = context.as_ref().unwrap();
        let game_settings = world.res.fetch::<settings::Config>().clone();

        // Create player
        world
            .create_entity()
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(300.0, -300.0, 0.0);
                transform.set_scale(
                    game_settings.graphics.scale,
                    game_settings.graphics.scale,
                    1.,
                );
                transform
            })
            .with(Player)
            .with(SpriteRender {
                sprite_sheet: map_sprite_sheet_handle.clone(),
                sprite_number: 25,
            })
            .with(TimeAvailable::default())
            .with(Actionable::default())
            .with(Transparent)
            .with(Rgba::RED)
            .build();

        // Create camera
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                -1000.0, 1000.0, -1000.0, 1000.0,
            )))
            .with({
                let mut transform = Transform::default();
                transform.set_translation_z(1.0);
                transform
            })
            .build();

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
            if let Some(sheet) = sheet.get(map_sprite_sheet_handle) {
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
        }

        state.data.update(state.world);
        Trans::None
    }
}

pub static mut SHEET_INIT: AtomicBool = AtomicBool::new(false);
