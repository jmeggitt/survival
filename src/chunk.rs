use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{ReadExpect, System, WriteExpect, };
use derivative::Derivative;
use hashbrown::HashMap;
use log::{error, info};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use shred_derive::SystemData;
use specs_derive::Component;

use crate::components::PlayerPosition;
use crate::tiles::TileId;
use shred::DynamicSystemData;

#[derive(Clone, Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Chunk {
    #[serde(skip_serializing, skip_deserializing)]
    pos: (i32, i32),
    #[derivative(Debug = "ignore")]
    pub tiles: [[TileId; 16]; 16],
    path: PathBuf,
}

impl Chunk {
    /// Save this chunk to the previous file
    pub fn save(&self) {
        info!("Saving chunk at ({}, {})", self.pos.0, self.pos.1);

        if self.path.exists() && !self.path.is_file() {
            error!("Unable to overwrite directory to save chunk!");
            return;
        }

        let mut file = match File::create(self.path.clone()) {
            Ok(v) => v,
            Err(_) => {
                error!("Unable to open file to save chunk");
                return;
            }
        };

        let serial = match to_string_pretty(
            &self,
            PrettyConfig {
                depth_limit: 99,
                separate_tuple_members: true,
                enumerate_arrays: true,
                ..PrettyConfig::default()
            },
        ) {
            Ok(v) => v,
            Err(_) => {
                error!("Unable to serialize chunk!");
                return;
            }
        };

        if let Err(e) = file.write(serial.as_bytes()) {
            error!("Unable to write to file {:?} due to {}", self.path, e);
        }
    }

    pub fn read<P: AsRef<Path>>(_: &P, _: (i32, i32)) -> Option<Self> {
        error!("I didn't feel like implementing file io yet. :(");
        None
    }

    pub fn load<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> Self {
        if let Some(chunk) = Chunk::read(path, pos) {
            return chunk;
        }

        // TODO generate
        Chunk {
            pos,
            tiles: [[TileId(20); 16]; 16],
            path: Chunk::file_name(path, pos),
        }
    }

    fn file_name<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> PathBuf {
        path.as_ref()
            .join(format!("chunk_{}x{}.save", pos.0, pos.1))
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        self.save()
    }
}

#[derive(Component)]
pub struct WorldChunks {
    inner: HashMap<(i32, i32), Chunk>,
}

impl WorldChunks {
    pub fn new() -> Self {
        WorldChunks {
            inner: HashMap::new(),
        }
    }

    fn reload_chunks<P: AsRef<Path>>(&mut self, player: Vector2<f32>, save_path: &P) {
        // TODO add to config
        const CHUNK_RADIUS: i32 = 4;

        let player_chunk_x = (player.x / 16.0).floor() as i32;
        let player_chunk_y = (player.y / 16.0).floor() as i32;

        self.inner.retain(|&(x, y), _| {
            (player_chunk_x - x).abs() <= CHUNK_RADIUS && (player_chunk_y - y).abs() <= CHUNK_RADIUS
        });

        for x in player_chunk_x - CHUNK_RADIUS..player_chunk_x + CHUNK_RADIUS {
            for y in player_chunk_y - CHUNK_RADIUS..player_chunk_y + CHUNK_RADIUS {
                let chunk_pos = (x, y);
                if !self.inner.contains_key(&chunk_pos) {
                    self.inner
                        .insert(chunk_pos, Chunk::load(save_path, chunk_pos));
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct ChunkSystemData<'a> {
    chunks: WriteExpect<'a, WorldChunks>,
    player: ReadExpect<'a, PlayerPosition>,
}

#[derive(Debug)]
pub struct ChunkLoadSystem<P: AsRef<Path>> {
    player_previous: Vector2<f32>,
    player_offset: Vector2<f32>,
    save_path: P,
}

impl<P: AsRef<Path>> ChunkLoadSystem<P> {
    pub fn new(path: P) -> Self {
        ChunkLoadSystem {
            player_previous: Vector2::new(0.0, 0.0),
            player_offset: Vector2::new(200.0, 200.0),
            save_path: path,
        }
    }
}

impl<'a, P: AsRef<Path>> System<'a> for ChunkLoadSystem<P> {
    type SystemData = ChunkSystemData<'a>;

    fn setup(&mut self, res: &mut Resources) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);
        res.insert(WorldChunks::new())
    }

    fn run(&mut self, mut data: ChunkSystemData) {
        self.player_offset = self.player_offset + data.player.0 - self.player_previous;
        self.player_previous = data.player.0;

        // Don't attempt to reload if there hasn't been a notable change in chunks
        if self.player_offset.x.abs() < 16.0 && self.player_offset.y.abs() < 16.0 {
            return;
        }

        // Reset offset
        self.player_offset = Vector2::new(0.0, 0.0);
        data.chunks.reload_chunks(data.player.0, &self.save_path)
    }
}
