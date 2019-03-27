#![feature(custom_attribute)]

use amethyst::{
    assets::{HotReloadBundle, PrefabLoaderSystem},
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage},
    ui::UiBundle,
    utils::{application_root_dir, fps_counter::FPSCounterBundle, scene::BasicScenePrefab},
};
use log::info;

use actions::PlayerInputAction;
use chunk::ChunkLoadSystem;
pub use game_data::{GameDispatchers, SurvivalDataBuilder};

use crate::render::tiles::Pass;

pub mod goap;
#[allow(dead_code)]
pub mod mapgen;
pub mod system_chain;

pub mod assets;
pub mod components;
pub mod render;
pub mod settings;
pub mod states;
pub mod systems;
pub mod tiles;
pub mod utils;

pub mod actions;

pub mod inventory;

pub mod game_data;
pub mod initializers;

pub mod specs_static;

mod chunk;

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

pub fn run() -> amethyst::Result<()> {
    let root = application_root_dir()?.join("resources");

    let display_config = DisplayConfig::load(root.join("display_config.ron"));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0; 4], 1.0)
            .with_pass(Pass::new())
            .with_pass(DrawFlat2D::new())
            .with_pass(amethyst::ui::DrawUi::new())
            .with_pass(amethyst_imgui::DrawUi::default().docking()),
    );

    let game_config = settings::Config::load(root.join("game_settings.ron"));

    let game_data = SurvivalDataBuilder::new(None, display_config.clone(), game_config)
        .with_core_bundle(TransformBundle::new())?
        .with_core_bundle(
            InputBundle::<PlayerInputAction, PlayerInputAction>::new()
                .with_bindings_from_file(root.join("input.ron"))?,
        )?
        .with_core(
            systems::ImguiBeginFrameSystem::default(),
            "imgui_begin_frame",
            &[],
        )
        .with_core(
            systems::DebugSystem::default(),
            "debug",
            &["imgui_begin_frame"],
        )
        .with_core_bundle(HotReloadBundle::default())?
        .with_core_bundle(UiBundle::<PlayerInputAction, PlayerInputAction>::new())?
        .with_core(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_core_bundle(FPSCounterBundle::default())?
        .with_core_bundle(
            RenderBundle::new(pipe, Some(display_config.clone()))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[])
                .with_hide_hierarchy_system(),
        )?
        .with_core(
            systems::UiSystem::default(),
            "ui",
            &["imgui_begin_frame", "ui_loader"],
        )
        .with_core(
            systems::ui::InventoryWindowSystem::default(),
            "inventory_window_system",
            &["ui"],
        )
        .with_core(
            systems::ImguiEndFrameSystem::default(),
            "imgui_end_frame",
            &["imgui_begin_frame", "ui", "debug"],
        ) // All systems which use imgui must be here.
        .with_level(systems::DroppedItemSystem::default(), "ground_items", &[])
        .with_level(systems::WearingSystem::default(), "wearing", &[])
        .with_level(systems::InputSystem::default(), "input", &[])
        .with_level(systems::TilePositionSystem::default(), "tile_position", &[])
        .with_level(systems::MovementSystem::default(), "movement", &[])
        .with_level(systems::TimeSystem::default(), "time", &[])
        .with_level(
            ChunkLoadSystem::new(root.join("saves")),
            "chunk_loader",
            &[],
        );

    let mut game = Application::build(root, crate::states::FirstLoad::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    info!("Starting game loop");
    game.run();

    Ok(())
}
