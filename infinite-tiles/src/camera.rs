use crate::config;
use crate::types::ChunkPos;
use macroquad::prelude::*;

pub struct Camera {
    pub position: Vec2,
    pub viewport_size: Vec2,
    last_chunk: ChunkPos,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec2::ZERO,
            viewport_size: Vec2::new(screen_width(), screen_height()),
            last_chunk: ChunkPos { x: 0, y: 0 },
        }
    }

    pub fn update(&mut self, target_pos: Vec2) {
        // follow target with camera
        self.position = target_pos - self.viewport_size * 0.5;
        // update viewport in case of window resizing
        self.viewport_size = Vec2::new(screen_width(), screen_height());
    }

    pub fn chunk_changed(&mut self, current_pos: Vec2) -> bool {
        let current_chunk = ChunkPos::from_world_pos(current_pos.x, current_pos.y);
        if current_chunk != self.last_chunk {
            self.last_chunk = current_chunk;
            true
        } else {
            false
        }
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        world_pos - self.position
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        screen_pos + self.position
    }

    pub fn get_visible_range(&self) -> (Vec2, Vec2) {
        let start = self.position;
        let end = self.position + self.viewport_size;

        // closure to add one tile to ensure smooth scrolling
        (
            Vec2::new(
                start.x - config::TILE_SIZE as f32,
                start.y - config::TILE_SIZE as f32,
            ),
            Vec2::new(
                end.x - config::TILE_SIZE as f32,
                end.y - config::TILE_SIZE as f32,
            ),
        )
    }
}
