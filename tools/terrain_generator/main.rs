extern crate amethyst;
extern crate amethyst_imgui;
use std::collections::{HashSet, HashMap};

use amethyst::{
    assets::{Loader, AssetStorage, HotReloadBundle},
    ecs::{Write, ReadExpect, Entity},
    prelude::*,
    core::{Transform, TransformBundle},
    renderer::{Camera, Projection, Texture, PngFormat, TextureHandle, TextureMetadata, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, },
    utils::application_root_dir,
};

use amethyst_imgui::{ImguiState, imgui, imgui::{im_str}};
use survival::mapgen::{Point, CellData, Cell, IndexPoint, Generator, GeneratorConfig, IslandGeneratorSettings};

#[derive(Default)]
pub struct ImguiBeginFrameSystem;
impl ImguiBeginFrameSystem {
    pub fn open_frame<'ui>(
        &mut self,
        dimensions: &amethyst::renderer::ScreenDimensions,
        time: &amethyst::core::timing::Time,
        imgui_state: &mut Option<ImguiState>,
    ) -> Option<&'ui imgui::Ui<'ui>>
    {
        let dimensions: &amethyst::renderer::ScreenDimensions = &dimensions;
        let time: &amethyst::core::timing::Time = &time;

        if dimensions.width() <= 0. || dimensions.height() <= 0. {
            return None;
        }

        let imgui = match imgui_state {
            Some(x) => &mut x.imgui,
            _ => return None,
        };

        let frame = imgui.frame(imgui::FrameSize::new(f64::from(dimensions.width()), f64::from(dimensions.height()), 1.), time.delta_seconds());
        std::mem::forget(frame);
        unsafe { imgui::Ui::current_ui() }
    }
}
impl<'s> amethyst::ecs::System<'s> for ImguiBeginFrameSystem {
    type SystemData = (
        ReadExpect<'s, amethyst::renderer::ScreenDimensions>,
        ReadExpect<'s, amethyst::core::timing::Time>,
        Write<'s, Option<ImguiState>>,
    );

    fn run(&mut self, (dimensions, time, mut imgui_state, ): Self::SystemData) {
        self.open_frame(&dimensions, &time, &mut imgui_state);
    }
}


struct UiState {
    seed: imgui::ImString,
}
impl Default for UiState {
    fn default() -> Self {
        Self {
            seed: "balls".to_string().into(),
        }
    }
}

#[derive(Default)]
pub struct ImguiEndFrameSystem {
    state: UiState,
}
impl<'s> amethyst::ecs::System<'s> for ImguiEndFrameSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        if let Some(ui) = unsafe { imgui::Ui::current_ui() } {
            unsafe {
                (ui as *const imgui::Ui).read_volatile();
                let root_dock = ui.dockspace_over_viewport(None, imgui::ImGuiDockNodeFlags::PassthruDockspace );
                //ui.show_demo_window(&mut true);
            }

            ui.window(imgui::im_str!("Generate Terrain"))
                .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    if ui.button(im_str!("Regenerate Island"), (0.0, 0.0)) {
                        use sha2::{Sha256, Digest};
                        use std::ops::Deref;

                        let mut hasher = Sha256::new();
                        hasher.input(self.state.seed.to_str().as_bytes());
                        let result = hasher.result();
                        generate_new_map(arrayref::array_ref![result.deref(), 0, 32]);
                    }
                    ui.input_text(im_str!("Seed"), &mut self.state.seed).build();

                });
        }
    }

}

struct Example;
impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let texture_handle = load_texture(world, "map.png");
        let _image = init_image(world, &texture_handle);

        init_camera(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> Trans<GameData<'static, 'static>, StateEvent> {
        amethyst_imgui::handle_imgui_events(data.world, &event);

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir()?.join("tools/terrain_generator/resources");
    let config = DisplayConfig::load(resources.join("display_config.ron"));
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(amethyst_imgui::DrawUi::default().docking()),
    );

    let game_data = GameDataBuilder::default()
        .with(ImguiBeginFrameSystem::default(), "imgui_begin_frame", &[])
        .with(ImguiEndFrameSystem::default(), "imgui_end_frame", &["imgui_begin_frame"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(HotReloadBundle::default())?;

    let mut game = Application::build(resources, Example)?.build(game_data)?;
    game.run();

    Ok(())
}

fn generate_new_map(seed : &[u8; 32],) -> amethyst::Result<()> {
    use rand::SeedableRng;

    let mut generator = Generator::new(rand::rngs::StdRng::from_seed(*seed));

    let config = GeneratorConfig {
        box_size: 500.0,
        num_points: 8000,
        ..Default::default()
    };

    let mut cells = generator.gen_voronoi::<CellData>(
        &config
    );
    generator.create_island(&config,
                            &IslandGeneratorSettings::default(),
                            &mut cells);

    generator.save_heightmap_image(&config, &application_root_dir()?.join("tools/terrain_generator/resources/map.png"), &cells).unwrap();

    Ok(())
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, -250.0, 250.0,
        )))
        .with(transform)
        .build();
}

fn init_image(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(0.0);
    transform.set_translation_y(0.0);

    world
        .create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}

fn load_texture(world: &mut World, png_path: &str) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        png_path,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
}