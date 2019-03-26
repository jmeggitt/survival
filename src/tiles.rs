use std::collections::HashSet;

use crate::specs_static::{Id, Storage};
use amethyst::{
    core::math::{Vector2, Vector3, Vector4},
    ecs::{Component, DenseVecStorage, Entity, Read, Write},
};
use serde::{Deserialize, Serialize};
use specs_derive::Component;

#[derive(Component, Clone, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct TileEntities(pub HashSet<Entity>);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TileId(pub u32);

impl TileId {
    #[inline]
    pub fn coords(self, dimensions: Vector2<u32>) -> (f32, f32) {
        (
            (self.0 % dimensions.y) as f32,
            (self.0 / dimensions.y) as f32,
        )
    }
}

impl Id for TileId {
    fn from_u32(value: u32) -> Self {
        Self(value)
    }

    fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Tiles {
    dimensions: Vector2<u32>,
}

impl Tiles {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            dimensions: Vector2::new(width, height),
        }
    }

    pub fn id(self, x: u32, y: u32) -> TileId {
        TileId(y * self.dimensions.y + x)
    }

    pub fn id_from_vector(self, vector: Vector2<u32>) -> TileId {
        TileId(vector.y * self.dimensions.y + vector.x)
    }

    pub fn world_to_tile(
        self,
        vector: &Vector3<f32>,
        game_settings: &crate::settings::Config,
    ) -> Vector2<u32> {
        Vector2::new(
            (vector.x / 20. / game_settings.graphics.scale) as u32,
            (vector.y / 20. / game_settings.graphics.scale).abs() as u32,
        )
    }

    pub fn world_to_id(
        self,
        vector: &Vector3<f32>,
        game_settings: &crate::settings::Config,
    ) -> TileId {
        self.id_from_vector(Vector2::new(
            (vector.x / 20. / game_settings.graphics.scale) as u32,
            (vector.y / 20. / game_settings.graphics.scale).abs() as u32,
        ))
    }

    pub fn iter_all(self) -> impl Iterator<Item = TileId> {
        (0..self.dimensions.x * self.dimensions.y - 1).map(TileId)
    }

    pub fn iter_region(self, region: Vector4<u32>) -> impl Iterator<Item = TileId> {
        RegionIter::new(self, region)
    }

    pub fn dimensions(self) -> Vector2<u32> {
        self.dimensions
    }
}

pub struct RegionIter {
    region: Vector4<u32>,
    tiles: Tiles,
    cur: Vector2<u32>,
    stride: u32,
}

impl RegionIter {
    pub fn new(tiles: Tiles, region: Vector4<u32>) -> Self {
        Self {
            stride: 1,
            region,
            tiles,
            cur: Vector2::new(region.x, region.y),
        }
    }
}

impl Iterator for RegionIter {
    type Item = TileId;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.x += self.stride;
        if self.cur.x > self.region.z {
            self.cur.x = self.region.x;
            self.cur.y += self.stride;
        }

        if self.cur.y > self.region.w {
            return None;
        }

        Some(self.tiles.id_from_vector(self.cur))
    }
}

pub type ReadTiles<'a, C> = Read<'a, Storage<C, <C as Component>::Storage, TileId>>;
pub type WriteTiles<'a, C> = Write<'a, Storage<C, <C as Component>::Storage, TileId>>;
