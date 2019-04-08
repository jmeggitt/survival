use amethyst::core::math::Vector2;
use amethyst::{assets::Handle, ecs::prelude::*, renderer::SpriteSheetHandle, shrev::EventChannel};
use bitflags::*;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

use crate::utils::HasChannel;

#[derive(Component, Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct PawnTraits {
    pub quickness: f32,
    pub move_speed: f32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Actionable {
    #[serde(skip)]
    pub channel: EventChannel<crate::actions::Action>,
}

impl Component for Actionable {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl HasChannel<crate::actions::Action> for Actionable {
    fn channel(&self) -> &EventChannel<crate::actions::Action> {
        &self.channel
    }

    fn channel_mut(&mut self) -> &mut EventChannel<crate::actions::Action> {
        &mut self.channel
    }
}

#[derive(
    Clone, Debug, PartialEq, Deserialize, Serialize, strum_macros::EnumString, strum_macros::Display,
)]
pub enum TreeFamily {
    Deciduous,
    Coniferous,
}

#[derive(
    Clone, Debug, PartialEq, Deserialize, Serialize, strum_macros::EnumString, strum_macros::Display,
)]
pub enum TreeKind {
    Pine,
    //(TreeFamily::Coniferous),
    Fur,
    //(TreeFamily::Coniferous),
    Spruce,
    //(TreeFamily::Coniferous),
    Cedar, //(TreeFamily::Coniferous),

    Oak,
    //(TreeFamily::Deciduous),
    Elm,
    //(TreeFamily::Deciduous),
    Maple,
    //(TreeFamily::Deciduous),
    Birch,
    //(TreeFamily::Deciduous),
    Willow, //(TreeFamily::Deciduous)
}

#[derive(
    Component,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum ObstructionType {
    Impassable,
    Blocking { height: f32 },
    Vegetation(f32),
    Liquid { depth: f32, current: f32 },
}

#[derive(Component, Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[storage(NullStorage)]
pub struct Impassable;

#[derive(Component, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[storage(DenseVecStorage)]
pub struct Tree {
    kind: TreeKind,
    size: f32,
    branches: f32,
}

#[derive(Component, Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[storage(DenseVecStorage)]
pub struct TimeAvailable(pub u64);

impl TimeAvailable {
    pub fn has(&self, time: u64) -> bool {
        self.0 >= time
    }
    pub fn consume(&mut self, time: u64) {
        self.0 -= time;
    }
    pub fn add(&mut self, time: u64) {
        self.0 += time;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlaggedSpriteRender {
    /// Handle to the sprite sheet of the sprite
    pub handle: SpriteSheetHandle,
    /// Index of the sprite on the sprite sheet
    pub sprite_number: usize,
}

impl Component for FlaggedSpriteRender {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Container;

#[derive(Component, Clone, Debug)]
#[storage(DenseVecStorage)]
pub struct Item {
    pub handle: Handle<crate::assets::item::Details>,
    pub properties: Vec<crate::assets::item::Property>,
}

impl PartialEq<Item> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct InteractionType: u64 {
        const NONE =                0;
        const CHOP =                1 << 1;
        const PICKUP =              1 << 2;
        const DIG  =                1 << 3;
        const HIT =                 1 << 4;
        const IGNITE =              1 << 5;
        const CUT =                 1 << 6;
        const HAMMER =              1 << 7;
    }
}

#[derive(Component, Default, Copy, Clone, Debug, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Interactable(InteractionType);

pub struct MaterialStatus {
    // TODO: Deterioration? Damage? HP?
}

#[derive(Component, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[storage(DenseVecStorage)]
pub struct Wearing;

#[derive(Component, Clone, Copy, Debug)]
pub struct PlayerPosition(pub Vector2<f32>);
