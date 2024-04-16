use macroquad::prelude::*;

#[macroquad::main("SNAKE-RS")]
async fn main() {
    request_new_screen_size(600f32, 640f32);

    let square_size = 25.0;

    loop {
        clear_background(BLACK);
        draw_text("SCORE:", 25.0, 25.0, 30.0, PURPLE);
        draw_text("0", 25.0, 45.0, 30.0, PURPLE);
        draw_text("BEST SCORE:", 430.0, 25.0, 30.0, PURPLE);
        draw_text("0", 430.0, 45.0, 30.0, PURPLE);

        draw_rectangle(24.0, 64.0, 552.0, 552.0, GRAY);
        draw_rectangle(25.0, 65.0, 550.0, 550.0, BLACK);

        render_mesh(25.0, 65.0, square_size, 550.0);

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