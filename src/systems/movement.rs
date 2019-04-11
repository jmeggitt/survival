use amethyst::core::math::Vector2;
use amethyst::{
    core::components::Transform,
    ecs::{
        Entities, Join, Read, ReadExpect, ReadStorage, Resources, SystemData, WriteExpect,
        WriteStorage,
    },
};
use log::{error, warn};

use crate::actions::{Action, Direction};
use crate::components;
use crate::components::PlayerPosition;
use crate::settings::Config;
use crate::tiles::{ReadTiles, Tiles};
use crate::utils::ComponentEventReader;

#[derive(Default)]
pub struct System {
    action_reader: ComponentEventReader<components::Actionable, Action>,
}

impl<'s> amethyst::ecs::System<'s> for System {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'s, Config>,
        ReadExpect<'s, Tiles>,
        Entities<'s>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::TimeAvailable>,
        WriteStorage<'s, components::Actionable>,
        WriteStorage<'s, Transform>,
        // Tile storages
        ReadTiles<'s, components::Impassable>,
        WriteExpect<'s, PlayerPosition>,
    );

    fn run(
        &mut self,
        (
            game_config,
            tiles,
            entities,
            players,
            mut times,
            mut actionables,
            mut transforms,
            tile_impassable,
            mut player_position,
        ): Self::SystemData,
    ) {
        self.action_reader.maintain(&entities, &mut actionables);

        // Read components...
        for (entity, time_comp, actionable, transform) in
            (&entities, &mut times, &mut actionables, &mut transforms).join()
        {
            for event in self.action_reader.read(entity, actionable) {
                if let Action::Move(direction) = event {
                    if crate::systems::time::has_time(1, entity, time_comp, &players) {
                        // Once its confirmed they can do it, run it
                        crate::systems::time::consume_time(1, entity, time_comp, &players);

                        // And finally, move one tile in the given direction
                        let mut target = transform.clone();

                        const SPEED: f32 = 6.0;

                        match direction {
                            Direction::N => {
                                target.move_up(SPEED);
                            }
                            Direction::S => {
                                target.move_down(SPEED);
                            }
                            Direction::E => {
                                target.move_right(SPEED);
                            }
                            Direction::W => {
                                target.move_left(SPEED);
                            }
                            _ => error!("Unsupported direction!"),
                        }

                        // Can we actually go to the target?
                        let target_tile = tiles.world_to_tile(target.translation(), &game_config);

                        if tile_impassable
                            .get(tiles.id_from_vector(target_tile))
                            .is_some()
                        {
                            // We cant do the move!
                            warn!(
                                "Attempted to move to impassable tile: ({},{})",
                                target_tile.x, target_tile.y
                            );
                            continue;
                        }

                        *transform = target;

                        if players.get(entity).is_some() {
                            player_position.0 = Vector2::new(0.0, 0.0);
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.action_reader.setup(res);
    }
}
