use amethyst::{
    ecs::{
        prelude::*,
    },
    renderer::SpriteSheetHandle,
    core::{
        components::Transform,
        nalgebra::Vector3
    },
    shrev::EventChannel,
};
use std::sync::Arc;
use std::collections::HashSet;
use specs_derive::Component;
use serde::{Serialize, Deserialize};
use bitflags::*;


#[derive(Component, Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Player;

pub struct ActionEvent;

#[derive(Default, Serialize, Deserialize)]
pub struct Actionable {
    #[serde(skip_serializing, skip_deserializing)]
    channel: EventChannel<ActionEvent>,
}
impl Component for Actionable {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlaggedSpriteRender {
    /// Handle to the sprite sheet of the sprite
    pub sprite_sheet: SpriteSheetHandle,
    /// Index of the sprite on the sprite sheet
    pub sprite_number: usize,
}

impl Component for FlaggedSpriteRender {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Container {
    #[serde(skip_serializing, skip_deserializing)]
    pub items: Vec<Entity>,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Item {
    #[serde(skip_serializing, skip_deserializing)]
    pub parent: Option<Entity>,

    pub details: Arc<crate::assets::ItemDetails>,
    pub properties: Vec<crate::assets::ItemProperty>,
}
impl PartialEq<Item> for Item { fn eq(&self, other: &Self) -> bool { self.details.name == other.details.name } }

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct TilePosition {
    pub coord: Vector3<u32>,
}
impl Default for TilePosition {
    fn default() -> Self { Self { coord: Vector3::new(0, 0, 0), } }
}
impl TilePosition {
    pub fn new(coord: Vector3<u32>) -> Self { Self { coord, } }
    pub fn from_transform(transform: &Transform, tiles: crate::tiles::Tiles, game_settings: &crate::settings::Config) -> Self {
        let position = tiles.world_to_tile(transform.translation(), &game_settings);;
        Self {
            coord: Vector3::new(position.x as u32, position.y as u32, 0),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct TileEntities(pub HashSet<Entity>);

bitflags_serial! {
    pub struct InteractionType: u64 {
        const None =                0;
        const Chop =                1 << 1;
        const Pickup =              1 << 2;
        const Dig  =                1 << 3;
        const Hit =                 1 << 4;
        const LightFire =           1 << 5;
        const Cut =                 1 << 6;
        const Hammer =              1 << 7;

    }
}

#[derive(Component, Default, Copy, Clone, Debug, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Interactable(InteractionType);

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ObstructionType {
    Impassable,
    Blocking { height: f32, },
    Vegetation(f32),
    Liquid { depth: f32, current: f32, }
}

pub struct MaterialStatus {
    // TODO: Deterioration? Damage? HP?
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Obstruction {
    pub kind: ObstructionType,
}