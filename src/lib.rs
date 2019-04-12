#![feature(custom_attribute, drain_filter)]
// Honestly, I don't really care about this clippy requirement and its a pain to have to fix all
// instances.
#![allow(clippy::cast_lossless)]

use amethyst::{
    assets::{HotReloadBundle, PrefabLoaderSystem},
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::{application_root_dir, fps_counter::FPSCounterBundle, scene::BasicScenePrefab},
};
use log::info;

use actions::PlayerInputAction;
pub use game_data::{GameDispatchers, SurvivalDataBuilder};
use systems::chunk::ChunkLoadSystem;

mod entity;
#[cfg(feature = "mapgen")]
pub mod mapgen;

pub mod assets;
pub mod components;
pub mod events;
pub mod render;
pub mod settings;
pub mod systems;
pub mod tiles;
pub mod utils;

pub mod actions;

pub mod inventory;

pub mod game_data;
pub mod initializers;

pub mod specs_static;

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

pub fn run() -> amethyst::Result<()> {
    let root = application_root_dir()?.join("resources");

    let display_config = DisplayConfig::load(root.join("display_config.ron"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(crate::render::tile_pass::TileRenderPass)
            .with_pass(DrawFlat2D::new()),
    );

    let game_config = settings::Config::load(root.join("game_settings.ron"));

    let render_bundle = RenderBundle::new(pipe, Some(display_config.clone()))
        .with_sprite_sheet_processor()
        .with_sprite_visibility_sorting(&[])
        .with_hide_hierarchy_system();

    let game_data = SurvivalDataBuilder::new(None, display_config.clone(), game_config)
        .with_core_bundle(TransformBundle::new())?
        .with_core_bundle(
            InputBundle::<PlayerInputAction, PlayerInputAction>::new()
                .with_bindings_from_file(root.join("input.ron"))?,
        )?
        .with_core_bundle(HotReloadBundle::default())?
        .with_core(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_core_bundle(FPSCounterBundle::default())?
        .with_level(systems::DroppedItemSystem::default(), "ground_items", &[])
        .with_level(systems::WearingSystem::default(), "wearing", &[])
        .with_level(systems::InputSystem::default(), "input", &[])
        .with_level(systems::MovementSystem::default(), "movement", &[])
        .with_level(systems::TimeSystem::default(), "time", &[])
        .with_level(
            ChunkLoadSystem::new(root.join("saves")),
            "chunk_loader",
            &[],
        )
                .with_level(
                    systems::entity_chunk::EntityChunkSystem::new(),
                    "entity_chunk",
                    &[],
                )
        .with_core_bundle(render_bundle)?;

    let mut game = Application::build(root, crate::events::FirstLoad::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    info!("Starting game loop");
    game.run();

    Ok(())
}
