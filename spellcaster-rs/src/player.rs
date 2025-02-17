use crate::camera::Camera;
use crate::config;
use crate::types::{ChunkPos, TilePos};
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub chunk_pos: ChunkPos,
    pub tile_pos: TilePos,
    velocity: Vec2,
}

impl Player {
    pub fn new() -> Self {
        // Start at world origin
        let position = Vec2::ZERO;
        Player {
            position,
            chunk_pos: ChunkPos::from_world_pos(position.x, position.y),
            tile_pos: TilePos::from_world_pos(position.x, position.y),
            velocity: Vec2::ZERO,
        }
    }

    pub fn update(&mut self) {
        // Handle input
        let mut input_dir = Vec2::ZERO;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            input_dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            input_dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            input_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            input_dir.x += 1.0;
        }

        // Normalize diagonal movement
        if input_dir != Vec2::ZERO {
            input_dir = input_dir.normalize();
        }

        // Apply movement
        self.velocity = input_dir * config::PLAYER_SPEED;
        self.position += self.velocity;

        // Update position trackers
        self.chunk_pos = ChunkPos::from_world_pos(self.position.x, self.position.y);
        self.tile_pos = TilePos::from_world_pos(self.position.x, self.position.y);
    }

    pub fn draw(&self, camera: &Camera) {
        // Convert world position to screen position
        let screen_pos = camera.world_to_screen(self.position);

        // Draw player
        draw_rectangle(
            screen_pos.x - config::PLAYER_SIZE / 2.0,
            screen_pos.y - config::PLAYER_SIZE / 2.0,
            config::PLAYER_SIZE,
            config::PLAYER_SIZE,
            BLUE,
        );

        // Debug info (optional)
        #[cfg(debug_assertions)]
        {
            draw_text(
                &format!("Chunk: ({}, {})", self.chunk_pos.x, self.chunk_pos.y),
                10.0,
                50.0,
                20.0,
                BLACK,
            );
            draw_text(
                &format!("Tile: ({}, {})", self.tile_pos.x, self.tile_pos.y),
                10.0,
                70.0,
                20.0,
                BLACK,
            );
        }
    }

    pub fn get_chunk_pos(&self) -> ChunkPos {
        self.chunk_pos
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }
}
