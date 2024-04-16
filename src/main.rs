use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(600f32, 600f32);

    loop {
        clear_background(BLACK);
        draw_text("SCORE: 0", 20.0, 20.0, 24.0, PURPLE);

        draw_rectangle(50.0, 50.0, 500.0, 500.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        next_frame().await
    }
}