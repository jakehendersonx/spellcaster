use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib.init().size(800, 600).title("wizard game").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("Hello, Raylib!", 350, 280, 20, Color::DARKGRAY);
    }
}
