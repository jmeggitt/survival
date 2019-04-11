use amethyst::ecs::{System, WriteExpect};
use log::debug;

use crate::entity::WorldEntity;
use crate::systems::chunk::WorldChunks;

// TODO offload out of bounds entities
pub struct EntityChunkSystem {
    transfer: Vec<WorldEntity>,
    offload: Vec<WorldEntity>,
}

impl EntityChunkSystem {
    pub fn new() -> Self {
        Self {
            transfer: Vec::with_capacity(16),
            offload: Vec::with_capacity(16),
        }
    }
}

impl<'a> System<'a> for EntityChunkSystem {
    type SystemData = WriteExpect<'a, WorldChunks>;

    fn run(&mut self, mut data: WriteExpect<'a, WorldChunks>) {
        // Remove all entities that are in the wrong chunk
        for ((chunk_x, chunk_y), ref mut chunk) in data.inner.iter_mut() {
            self.transfer.extend(chunk.entities.drain_filter(|e| {
                (e.pos.x / 16.0).floor() as i32 != *chunk_x
                    || (e.pos.y / 16.0).floor() as i32 != *chunk_y
            }));
        }

        // Re-add them to either the correct chunk or offload queue
        for entity in self.transfer.drain(..) {
            let chunk_pos = (
                (entity.pos.x / 16.0).floor() as i32,
                (entity.pos.y / 16.0).floor() as i32,
            );
            match data.inner.get_mut(&chunk_pos) {
                Some(v) => v.entities.push(entity),
                None => {
                    self.offload.push(entity);
                    debug!("Entity added to offload queue: {}", self.offload.len());
                }
            }
        }
    }
}
