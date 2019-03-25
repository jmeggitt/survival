use amethyst::{
    core::components::{GlobalTransform, Transform},
    ecs::{Builder, Entity, SystemData, World},
    renderer::{Camera, Projection, Rgba, SpriteRender, SpriteSheetHandle, Transparent},
    StateData, StateEvent, Trans,
};
use log::trace;

use crate::components::{Actionable, FlaggedSpriteRender, Player, TilePosition, TimeAvailable};
use crate::settings;
use crate::states::Paused;
use crate::tiles::TileEntities;
use crate::tiles::{Tiles, WriteTiles};
use crate::SurvivalData;

fn init_player(
    world: &mut World,
    sprite_sheet: &SpriteSheetHandle,
    tiles: Tiles,
    game_settings: &settings::Config,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(100.0);
    transform.set_translation_y(-100.0);
    transform.set_scale(
        game_settings.graphics.scale,
        game_settings.graphics.scale,
        1.,
    );
    world
        .create_entity()
        .with(TilePosition::from_transform(
            &transform,
            tiles,
            game_settings,
        ))
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
    //*transform.scale_mut() = transform.scale() * 4.0;
    // transform.set_scale(game_settings.graphics.scale * 20. * -1., game_settings.graphics.scale * 20. * -1., 0.);
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
        //.with(Parent { entity: parent })
        .with(transform)
        .build();
}

pub struct Level;

impl<'a, 'b> amethyst::State<SurvivalData<'a, 'b>, StateEvent> for Level {
    fn on_start(&mut self, data: StateData<'_, SurvivalData<'_, '_>>) {
        let world = data.world;
        trace!("Changed state to Level");

        // Load the level
        let tiles = Tiles::new(100, 100);
        {
            let context = world.res.fetch::<settings::Context>().clone();
            let map_sprite_sheet_handle = context.as_ref().unwrap();
            let game_settings = world.res.fetch::<settings::Config>().clone();

            let player = init_player(world, map_sprite_sheet_handle, tiles, &game_settings);
            init_camera(world, player, tiles, &game_settings);

            let mut sprites: WriteTiles<FlaggedSpriteRender> = SystemData::fetch(&world.res);
            let mut transforms: WriteTiles<GlobalTransform> = SystemData::fetch(&world.res);
            let mut tile_entities_map: WriteTiles<TileEntities> = SystemData::fetch(&world.res);
            //let tile_rgb: WriteTiles<Rgba> = SystemData::fetch(&world.res);
            for tile_id in tiles.iter_all() {
                tile_entities_map.insert_default(tile_id);

                sprites.insert(
                    tile_id,
                    FlaggedSpriteRender {
                        sprite_sheet: map_sprite_sheet_handle.clone(),
                        sprite_number: 11,
                    },
                );

                // tile_rgb.insert(tile_id, Rgba::GREEN);

                let coords = tile_id.coords(tiles.dimensions());
                let mut transform = Transform::default();

                let width = 16.;
                let height = 16.;
                transform.set_translation_xyz(
                    coords.0 * width * game_settings.graphics.scale,
                    -1. * (coords.1 * height * game_settings.graphics.scale),
                    0.,
                );
                transform.set_scale(
                    game_settings.graphics.scale,
                    game_settings.graphics.scale,
                    game_settings.graphics.scale,
                );

                let mut global = GlobalTransform::default();
                global.0 = transform.matrix();
                transforms.insert(tile_id, global);
            }

            let mut impassable_tiles: WriteTiles<crate::components::Impassable> =
                SystemData::fetch(&world.res);
            // Set all the edges to impassable
            for x in &[0, tiles.dimensions().x - 1] {
                for y in 0..tiles.dimensions().y {
                    impassable_tiles.insert_default(tiles.id(*x, y));
                }
            }
            for y in &[0, tiles.dimensions().y - 1] {
                for x in 0..tiles.dimensions().x {
                    impassable_tiles.insert_default(tiles.id(x, *y));
                }
            }
        }

        world.add_resource(tiles);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, SurvivalData<'_, '_>>,
        _: StateEvent,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        trace!("Event Level");
        Trans::None
    }

    fn update(
        &mut self,
        _: StateData<'_, SurvivalData<'_, '_>>,
    ) -> Trans<SurvivalData<'a, 'b>, StateEvent> {
        // Just swap straight to Paused
        Trans::Push(Box::new(Paused))
    }
}
