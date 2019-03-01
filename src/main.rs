#![feature(custom_attribute)]
#![allow(dead_code)]

pub mod tiles;
pub mod render;
pub mod states;
pub mod components;
pub mod settings;

pub mod mapgen;

use amethyst::{
    core::{Transform, TransformBundle},
    ecs::{Component, Join, NullStorage, Read, ReadStorage, System, WriteStorage},
    input::{InputBundle, InputHandler},
    prelude::*,
    renderer::{
        DrawFlat2D, DisplayConfig, Pipeline, RenderBundle, Stage,
    },
    utils::application_root_dir,
};

#[derive(Default)]
struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.translate_x(x_move as f32 * 20.0);
            transform.translate_y(y_move as f32 * 20.0);
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?.join("resources");

    let display_config = DisplayConfig::load(root.join("display_config.ron"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(crate::render::tiles::Pass::new())
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(root.join("input.ron"))?,
        )?
        .with(MovementSystem, "movement", &[])
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;

    let mut game = Application::build(root, crate::states::Example)?.build(game_data)?;


    game.run();
    Ok(())
}
