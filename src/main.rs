use std::{sync::Arc, time::Duration};

use board::Board;
use game::Game;
use macroquad::prelude::*;
use snake::Snake;

mod board;
mod snake;
mod game;

#[macroquad::main("SNAKE-RS")]
async fn main() {
    request_new_screen_size(550f32, 620f32);
    let mut game = Game::default();
    let board = Board::new(500.0, 21, 25.0, 60.0);
    let mut snake = Snake::new(&board, Duration::from_millis(200));

    loop {
        clear_background(BLACK);
        draw_text("SCORE:", 25.0, 25.0, 30.0, PURPLE);
        draw_text("0", 25.0, 45.0, 30.0, PURPLE);
        draw_text("BEST SCORE:", 380.0, 25.0, 30.0, PURPLE);
        draw_text("0", 380.0, 45.0, 30.0, PURPLE);

        board.render();
        snake.render(&board);
        snake.handle_frame(&mut game, &board);

        if game.is_over() {
            draw_text("GAME OVER", 135.0, 280.0, 72.0, YELLOW);
        }

        draw_text("W,A,S,D - control", 25.0, 580.0, 30.0, GREEN);
        draw_text("SPACE - pause", 355.0, 580.0, 30.0, GREEN);

        next_frame().await
    }
}
