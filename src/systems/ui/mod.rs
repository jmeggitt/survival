pub mod imgui;
pub mod ui;

pub mod inventory_window;
pub use inventory_window::System as InventoryWindowSystem;

pub type ImGuiDraw = std::sync::Arc<Fn(&amethyst_imgui::imgui::Ui, &amethyst::ecs::LazyUpdate) + Send + Sync>;