use macroquad::prelude::*;
use std::hash::Hash;

#[derive(PartialEq, Clone, Copy)]
pub enum LoadPriority {
    Immediate,
    Preload,
    Cache,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn from_world_pos(x: f32, y: f32) -> Self {
        ChunkPos {
            x: (x / (crate::config::CHUNK_SIZE * crate::config::TILE_SIZE) as f32).floor() as i32,
            y: (y / (crate::config::CHUNK_SIZE * crate::config::TILE_SIZE) as f32).floor() as i32,
        }
    }
}

impl TilePos {
    pub fn from_world_pos(x: f32, y: f32) -> Self {
        TilePos {
            x: (x / crate::config::TILE_SIZE as f32).floor() as i32,
            y: (y / crate::config::TILE_SIZE as f32).floor() as i32,
        }
    }
}
