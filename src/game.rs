use macroquad::{input::{get_last_key_pressed, KeyCode}, miniquad::window::quit};

use crate::{board::Board, snake::Snake};

#[derive(Default)]
pub struct Game {
    points: usize,
    best_result: usize,
    paused: bool,
    over: bool,
}

impl Game {
    pub fn handle_frame(&mut self, board: &mut Board, snake: &mut Snake) {
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Space => {
                    if self.is_over() {
                        snake.restart(board);
                        board.restart();
                        self.over = false;
                        self.points = 0;
                    } else {
                        self.paused = !self.paused;
                    }
                },
                KeyCode::Escape => {quit()},
                _ => (),
            }
        }
    }

    pub fn on_eat(&mut self) {
        self.points += 100;
    }

    pub fn on_game_over(&mut self) {
        if self.points > self.best_result {
            self.best_result = self.points;
        }
        self.over = true;
    }

    pub fn is_over(&self) -> bool {
        self.over
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_best_points(&self) -> usize {
        self.best_result
    }
}