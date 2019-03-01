use amethyst::ecs::prelude::*;
use amethyst::renderer::SpriteSheetHandle;

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