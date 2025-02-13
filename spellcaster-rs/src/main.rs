use macroquad::prelude::*;

const GRID_SIZE: i32 = 20; // Size of each grid cell in pixels
const GRID_WIDTH: i32 = 30; // Number of cells horizontally
const GRID_HEIGHT: i32 = 20; // Number of cells vertically
const PLAYER_SIZE: f32 = 18.0; // Size of the player square

struct Player {
    grid_x: i32,
    grid_y: i32,
}

impl Player {
    fn new() -> Self {
        // center player on the grid
        Player {
            grid_x: GRID_WIDTH / 2,
            grid_y: GRID_HEIGHT / 2,
        }
    }

    fn move_direction(&mut self, dx: i32, dy: i32) {
        let new_x = self.grid_x + dx;
        let new_y = self.grid_y + dy;

        // Check boundaries
        if new_x >= 0 && new_x < GRID_WIDTH && new_y >= 0 && new_y < GRID_HEIGHT {
            self.grid_x = new_x;
            self.grid_y = new_y;
        }
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            self.move_direction(0, -1);
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.move_direction(0, 1);
        }
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            self.move_direction(-1, 0);
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            self.move_direction(1, 0);
        }
    }

    fn draw(&self) {
        let screen_x = (self.grid_x * GRID_SIZE) as f32;
        let screen_y = (self.grid_y * GRID_SIZE) as f32;

        draw_rectangle(
            screen_x + (GRID_SIZE as f32 - PLAYER_SIZE) / 2.0,
            screen_y + (GRID_SIZE as f32 - PLAYER_SIZE) / 2.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
            BLUE,
        );
    }
}

fn draw_grid() {
    for i in 0..=GRID_WIDTH {
        let x = (i * GRID_SIZE) as f32;
        draw_line(x, 0.0, x, (GRID_HEIGHT * GRID_SIZE) as f32, 1.0, LIGHTGRAY);
    }

    for i in 0..=GRID_HEIGHT {
        let y = (i * GRID_SIZE) as f32;
        draw_line(0.0, y, (GRID_WIDTH * GRID_SIZE) as f32, y, 1.0, LIGHTGRAY);
    }
}

#[macroquad::main("wizard game")]
async fn main() {
    let mut player = Player::new();
    loop {
        clear_background(WHITE);
        player.update(); // logic for positional updates inside struct
                         // we can contain our draw logic inside the structs
        draw_grid();
        player.draw();
        next_frame().await;
    }
}
