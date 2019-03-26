use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use log::{error, info};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::tiles::TileId;

fn chunk_file_name<P: AsRef<Path>>(path: &P, pos: (i32, i32)) -> PathBuf {
    path.as_ref()
        .join(format!("chunk_{}x{}.save", pos.0, pos.1))
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pos: (i32, i32),
    pub tiles: [[TileId; 16]; 16], // Data
}

impl Chunk {
    /// Consume this chunk in save
    pub fn save<P: AsRef<Path>>(self, path: &P) {
        info!("Saving chunk at ({}, {})", self.pos.0, self.pos.1);

        let path = chunk_file_name(path, self.pos);

        if path.exists() && !path.is_file() {
            error!("Unable to overwrite directory to save chunk!");
            return;
        }

        let mut file = match File::create(path.clone()) {
            Ok(v) => v,
            Err(e) => {
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
            Err(e) => {
                error!("Unable to serialize chunk!");
                return;
            }
        };

        if let Err(e) = file.write(serial.as_bytes()) {
            error!("Unable to write to file: {:?}", path);
        }
    }

    pub fn read<P: AsRef<Path>>(path: P, pos: (i32, i32)) -> Option<Self> {
        error!("I didn't feel like implementing file io yet. :(");
        None
    }

    pub fn load<P: AsRef<Path>>(path: P, pos: (i32, i32)) -> Self {
        if let Some(chunk) = Chunk::read(path, pos) {
            return chunk;
        }

        // TODO generate
        Chunk {
            pos,
            tiles: [[TileId(20); 16]; 16],
        }
    }
}
