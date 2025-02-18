// world.rs
use crate::camera::Camera;
use crate::config;
use crate::texture::TextureManager;
use crate::types::{ChunkPos, LoadPriority};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Chunk {
    pub pos: ChunkPos,
    tiles: Vec<Vec<u32>>, // Stores tile IDs
}

impl Chunk {
    fn new(pos: ChunkPos) -> Self {
        let mut tiles = Vec::with_capacity(config::CHUNK_SIZE as usize);
        for _ in 0..config::CHUNK_SIZE {
            let row = (0..config::CHUNK_SIZE)
                .map(|_| rand::gen_range::<u32>(0, 4))
                .collect();
            tiles.push(row);
        }

        Chunk { pos, tiles }
    }
}

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    texture_manager: TextureManager,
}

impl World {
    pub async fn new() -> Self {
        World {
            chunks: HashMap::new(),
            texture_manager: TextureManager::new().await,
        }
    }

    pub async fn update(&mut self, center_chunk: ChunkPos) {
        // Calculate chunk loading ranges
        let ranges = [
            (config::VISIBLE_CHUNKS_RADIUS, LoadPriority::Immediate),
            (config::PRELOAD_CHUNKS_RADIUS, LoadPriority::Preload),
            (config::CACHE_CHUNKS_RADIUS, LoadPriority::Cache),
        ];

        // Track which chunks we want to keep
        let mut chunks_to_keep = Vec::new();

        // Load or update chunks in each range
        for &(radius, priority) in &ranges {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let chunk_pos = ChunkPos {
                        x: center_chunk.x + dx,
                        y: center_chunk.y + dy,
                    };

                    chunks_to_keep.push(chunk_pos);

                    // Create chunk if it doesn't exist
                    if !self.chunks.contains_key(&chunk_pos) {
                        self.chunks.insert(chunk_pos, Chunk::new(chunk_pos));
                    }

                    // Ensure textures are loaded with appropriate priority
                    let chunk_id = format!("chunk_{}_{}", chunk_pos.x, chunk_pos.y);
                    self.texture_manager
                        .ensure_loaded(&chunk_id, priority)
                        .await;
                }
            }
        }

        // Remove chunks that are too far away
        self.chunks.retain(|pos, _| chunks_to_keep.contains(pos));
    }

    pub fn draw(&self, camera: &Camera) {
        let (view_start, view_end) = camera.get_visible_range();

        // Convert view range to chunk coordinates
        let start_chunk = ChunkPos::from_world_pos(view_start.x, view_start.y);
        let end_chunk = ChunkPos::from_world_pos(view_end.x, view_end.y);

        // Draw visible chunks
        for chunk_y in start_chunk.y..=end_chunk.y {
            for chunk_x in start_chunk.x..=end_chunk.x {
                let chunk_pos = ChunkPos {
                    x: chunk_x,
                    y: chunk_y,
                };

                if let Some(chunk) = self.chunks.get(&chunk_pos) {
                    self.draw_chunk(chunk, camera);
                }
            }
        }
    }

    fn draw_chunk(&self, chunk: &Chunk, camera: &Camera) {
        let chunk_world_x = chunk.pos.x * config::CHUNK_SIZE * config::TILE_SIZE;
        let chunk_world_y = chunk.pos.y * config::CHUNK_SIZE * config::TILE_SIZE;

        for tile_y in 0..config::CHUNK_SIZE {
            for tile_x in 0..config::CHUNK_SIZE {
                let world_x = chunk_world_x + tile_x * config::TILE_SIZE;
                let world_y = chunk_world_y + tile_y * config::TILE_SIZE;

                let screen_pos = camera.world_to_screen(Vec2::new(world_x as f32, world_y as f32));

                // Only draw if on screen
                if screen_pos.x >= -config::TILE_SIZE as f32
                    && screen_pos.x <= screen_width()
                    && screen_pos.y >= -config::TILE_SIZE as f32
                    && screen_pos.y <= screen_height()
                {
                    // Get the appropriate texture for this tile
                    let texture = self.texture_manager.get_texture(world_x, world_y);

                    draw_texture(
                        Arc::as_ref(&texture), // Dereference the Arc to get the Texture2D
                        screen_pos.x,
                        screen_pos.y,
                        WHITE,
                    );
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn draw_debug_info(&self) {
        draw_text(
            &format!("Active Chunks: {}", self.chunks.len()),
            10.0,
            90.0,
            20.0,
            BLACK,
        );
    }
}
