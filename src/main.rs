#![warn(clippy::pedantic, clippy::all)]
#![allow(non_upper_case_globals)]

#![feature(custom_attribute, concat_idents)]
#![allow(dead_code)]


#[macro_use] pub mod bitflags_serial;

pub mod assets;
pub mod tiles;
pub mod render;
pub mod states;
pub mod components;
pub mod settings;
pub mod systems;

pub mod mapgen;

use amethyst::{
    core::{TransformBundle},
    input::{InputBundle},
    prelude::*,
    renderer::{
        DrawFlat2D, DisplayConfig, Pipeline, RenderBundle, Stage,
    },
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?.join("resources");

    let display_config = DisplayConfig::load(root.join("display_config.ron"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(crate::render::tiles::Pass::new())
            .with_pass(DrawFlat2D::new())
            .with_pass(amethyst_imgui::DrawUi::default()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(root.join("input.ron"))?,
        )?
        .with(systems::PlayerInputSystem::default(), "player_input", &[])
        .with(systems::ActionSystem::default(), "action", &[])
        .with_thread_local(systems::TimeSystem::default())
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]), // Let's us use the `Transparent` component
        )?;

    let mut game = Application::build(root, crate::states::Example::new())?.build(game_data)?;

    game.run();
    Ok(())
}
