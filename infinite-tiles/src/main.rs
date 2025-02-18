// main.rs
mod camera;
mod config;
mod player;
mod texture;
mod types;
mod world;

use camera::Camera;
use macroquad::prelude::*;
use player::Player;
use world::World;

#[macroquad::main("Infinite Tiles")]
async fn main() {
    let mut camera = Camera::new();
    let mut player = Player::new();
    let mut world = World::new().await;

    loop {
        clear_background(WHITE);

        // Update
        player.update();
        camera.update(player.get_position());

        // Update world if player moved to new chunk
        if camera.chunk_changed(player.get_position()) {
            world.update(player.get_chunk_pos()).await;
        }

        // Draw
        world.draw(&camera);
        player.draw(&camera);

        // Debug info
        #[cfg(debug_assertions)]
        {
            world.draw_debug_info();
            draw_text("WASD/Arrow Keys to move", 10.0, 30.0, 20.0, BLACK);
        }

        next_frame().await;
    }
}
