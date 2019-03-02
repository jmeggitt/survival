use amethyst::{
    ecs::prelude::*,
    renderer::SpriteSheetHandle,
};
use std::sync::Arc;
use specs_derive::Component;
use serde::{Serialize, Deserialize};


#[derive(Component, Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Player;

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
impl PartialEq<Item> for Item { fn eq(&self, other: &Item) -> bool { self.details.name == other.details.name } }