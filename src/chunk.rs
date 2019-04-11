use std::fs::{DirBuilder, File};
#[cfg(not(feature = "no-save"))]
use std::io::Write;
use std::path::{Path, PathBuf};

use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{ReadExpect, System, WriteExpect};
use derivative::Derivative;
use hashbrown::HashMap;
use log::{error, info, warn};
use ron::de::from_reader;
#[cfg(not(feature = "no-save"))]
use ron::ser::to_string;
#[cfg(all(feature = "pretty-save", not(feature = "no-save")))]
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use shred::DynamicSystemData;
use shred_derive::SystemData;
use specs_derive::Component;

use crate::components::PlayerPosition;
use crate::render::tile_pass::{compile_chunk, WriteChunkRender};
use crate::tiles::TileId;
use crate::tiles::{TileAsset, TileAssets};
use amethyst::assets::ProgressCounter;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Chunk {
    #[serde(skip)]
    pub pos: (i32, i32),
    #[derivative(Debug = "ignore")]
    #[serde(default = [[TileId(0); 16]; 16])]
    pub tiles: [[TileId; 16]; 16],
    #[serde(skip)]
    path: PathBuf,
    #[serde(skip)]
    requires_save: bool,
}

impl Chunk {
    /// Save this chunk to the previous file
    #[cfg(not(feature = "no-save"))]
    fn save(&self) {
        info!("Saving and unloading {:?}", self);

        if self.path.exists() && !self.path.is_file() {
            error!("Unable to overwrite directory to save chunk!");
            return;
        }

        let mut file = match File::create(self.path.clone()) {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to open {:?} to save chunk: {}", self.path, e);
                return;
            }
        };

        #[cfg(feature = "pretty-save")]
        let save = to_string_pretty(
            &self,
            PrettyConfig {
                depth_limit: 99,
                enumerate_arrays: true,
                ..PrettyConfig::default()
            },
        );

        #[cfg(not(feature = "pretty-save"))]
        let save = to_string(&self);

        let serial = match save {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to serialize chunk: {}", e);
                return;
            }
        };

        if let Err(e) = file.write(serial.as_bytes()) {
            error!("Unable to write to file {:?} due to {}", self.path, e);
        }
    }

    fn read<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> Option<Self> {
        if !path.as_ref().exists() {
            return None;
        }

        if !path.as_ref().is_file() {
            warn!("Could not read chunk {:?} due to non file at path", pos);
            return None;
        }

        let file_name = Chunk::file_name(path, pos);

        let file = match File::open(&*file_name.as_path()) {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to open file to read chunk: {}", e);
                return None;
            }
        };

        from_reader(file).ok()
    }

    /// Generate a new chunk from its coords and possibly more information in future.
    fn generate(pos: (i32, i32)) -> [[TileId; 16]; 16] {
        info!("Generating new chunk at {:?}", pos);
        let mut tiles = [[TileId(25); 16]; 16];

        for i in 0..16 {
            match i {
                0 | 15 => tiles[i] = [TileId(11); 16],
                _ => {
                    tiles[i][0] = TileId(11);
                    tiles[i][15] = TileId(11);
                }
            }
        }
        tiles
    }

    pub fn load<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> Self {
        let path = Chunk::file_name(path, pos);
        if let Some(mut found) = Chunk::read(&path, pos) {
            found.pos = pos;
            found.path = path;
            found
        } else {
            Chunk {
                pos,
                tiles: Chunk::generate(pos),
                path,
                requires_save: true,
            }
        }
    }

    fn file_name<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> PathBuf {
        path.as_ref()
            .join(format!("chunk_{}x{}.save", pos.0, pos.1))
    }
}

#[cfg(not(feature = "no-save"))]
impl Drop for Chunk {
    fn drop(&mut self) {
        if self.requires_save {
            self.save()
        }
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

    fn reload_chunks<P: AsRef<Path>>(
        &mut self,
        player: Vector2<f32>,
        save_path: &P,
        assets: &[TileAsset],
        renders: &mut WriteChunkRender,
    ) {
        info!("Performing chunk refresh");
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
                    log::debug!("Creating and adding chunk!");
                    let chunk = Chunk::load(save_path, chunk_pos);
                    renders.insert(chunk.pos, compile_chunk(&chunk, assets));
                    self.inner.insert(chunk_pos, chunk);
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct ChunkSystemData<'a> {
    chunks: WriteExpect<'a, WorldChunks>,
    player: ReadExpect<'a, PlayerPosition>,
    tile_assets: ReadExpect<'a, TileAssets>,
    chunk_renders: WriteChunkRender<'a>,
    // TODO Use progress counter instead of atomic bool
    #[allow(dead_code)]
    asset_progress: ReadExpect<'a, ProgressCounter>,
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
            player_offset: Vector2::new(2000.0, 2000.0),
            save_path: path,
        }
    }
}

use crate::events::SHEET_INIT;
use std::sync::atomic::Ordering;

impl<'a, P: AsRef<Path>> System<'a> for ChunkLoadSystem<P> {
    type SystemData = ChunkSystemData<'a>;

    fn run(&mut self, mut data: ChunkSystemData) {
        self.player_offset = self.player_offset + data.player.0 - self.player_previous;
        self.player_previous = data.player.0;

        // Don't attempt to reload if there hasn't been a notable change in chunks
        if self.player_offset.x.abs() < 16.0 && self.player_offset.y.abs() < 16.0
            || !unsafe { SHEET_INIT.load(Ordering::SeqCst) }
        {
            return;
        }

        //        use log::debug;
        //        debug!("Complete: {:?} {}/{}", data.asset_progress.complete(), data.asset_progress.num_assets(), data.asset_progress.num_finished());

        // Reset offset
        self.player_offset = Vector2::new(0.0, 0.0);
        data.chunks.reload_chunks(
            data.player.0,
            &self.save_path,
            &data.tile_assets.0,
            &mut data.chunk_renders,
        );
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("Setting up world chunks resource");

        #[cfg(feature = "pretty-save")]
        info!("Using pretty ron serialization");

        #[cfg(feature = "no-save")]
        info!("Using no save mode");

        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);
        res.insert(PlayerPosition(Vector2::new(0.0, 0.0)));
        res.insert(WorldChunks::new());

        if cfg!(not(feature = "no-save")) && !self.save_path.as_ref().exists() {
            info!("Creating save folder");
            let builder = DirBuilder::new();
            if let Err(e) = builder.create(self.save_path.as_ref()) {
                error!("Could not create save folder: {}", e);
            }
        }
    }
}
