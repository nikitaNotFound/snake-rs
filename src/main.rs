use macroquad::prelude::*;

#[macroquad::main("SNAKE-RS")]
async fn main() {
    request_new_screen_size(550f32, 620f32);

    let square_size = 25.0;

    loop {
        clear_background(BLACK);
        draw_text("SCORE:", 25.0, 25.0, 30.0, PURPLE);
        draw_text("0", 25.0, 45.0, 30.0, PURPLE);
        draw_text("BEST SCORE:", 380.0, 25.0, 30.0, PURPLE);
        draw_text("0", 380.0, 45.0, 30.0, PURPLE);

        draw_rectangle(24.0, 59.0, 502.0, 502.0, GRAY);
        draw_rectangle(25.0, 60.0, 500.0, 500.0, BLACK);

        render_mesh(25.0, 60.0, square_size, 500.0);
        draw_text("W,A,S,D - control", 25.0, 580.0, 30.0, GREEN);
        draw_text("SPACE - pause", 355.0, 580.0, 30.0, GREEN);

        next_frame().await
    }
}

fn render_mesh(x: f32, y: f32, square_size: f32, area_size: f32) {
    let mut x_index: usize = 0;
    let mut y_index: usize = 0;
    let squares_count = area_size as usize / square_size as usize;

    let x_point = x;
    let y_point = y;

    while y_index < squares_count {
        while x_index < squares_count {
            let x = x_point + square_size * x_index as f32;
            let y = y_point + square_size * y_index as f32;

            draw_rectangle(x, y, square_size, square_size, GRAY);
            draw_rectangle(x + 1.0, y + 1.0, square_size - 2.0, square_size - 2.0, BLACK);

            x_index += 1;
        }

        y_index += 1;
        x_index = 0;
    }
}