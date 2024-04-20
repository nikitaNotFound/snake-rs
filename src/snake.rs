use std::{collections::LinkedList, sync::Arc, time::{Duration, Instant}};

use macroquad::{color::RED, input::{get_last_key_pressed, KeyCode}};

use crate::{board::{Board, Point}, game::Game};

#[derive(Clone, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn get_opposite_dir(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn get_next_point(&self, point: &Point) -> Point {
        match self {
            Direction::Left => Point::new(point.x - 1, point.y),
            Direction::Right => Point::new(point.x + 1, point.y),
            Direction::Up => Point::new(point.x, point.y - 1),
            Direction::Down => Point::new(point.x, point.y + 1),
        }
    }

    pub fn get_prev_point(&self, point: &Point) -> Point {
        match self {
            Direction::Left => Point::new(point.x + 1, point.y),
            Direction::Right => Point::new(point.x - 1, point.y),
            Direction::Up => Point::new(point.x, point.y + 1),
            Direction::Down => Point::new(point.x, point.y - 1),
        }
    }
}

pub struct Snake {
    body: LinkedList<Point>,
    head: Point,
    direction: Direction,
    last_move_time: Option<Instant>,
    speed: Duration,
    navigation_lock: bool,
}

impl Snake {
    pub fn new(board: &Board, speed: Duration) -> Self {
        Self {
            head: board.get_center_point(),
            body: LinkedList::new(),
            direction: Direction::Up,
            last_move_time: None,
            speed,
            navigation_lock: false,
        }
    }

    pub fn handle_frame(&mut self, game: &mut Game, board: &Board) {
        if !self.navigation_lock {
            if let Some(key) = get_last_key_pressed() {
                match key {
                    KeyCode::A => {self.change_direction(Direction::Left)},
                    KeyCode::D => {self.change_direction(Direction::Right)},
                    KeyCode::W => {self.change_direction(Direction::Up)},
                    KeyCode::S => {self.change_direction(Direction::Down)},
                    _ => (),
                }
            }
        }

        if let None = self.last_move_time {
            self.last_move_time = Some(Instant::now());
        }

        if let Some(lmt) = self.last_move_time {
            if lmt.elapsed() > self.speed {
                let next_head = self.direction.get_next_point(&self.head);
                if let Some(_) = self.body.iter().find(|b| b.eq(&next_head)) {
                    game.on_game_over();
                    return;
                }

                if board.check_point_overflow(&next_head) {
                    game.on_game_over();
                    return;
                }

                self.body.push_front(self.head.clone());
                self.head = next_head;
                self.body.pop_back();

                self.last_move_time = Some(Instant::now());
                self.navigation_lock = false;
            }
        }
    }

    pub fn render(&self, board: &Board) {
        board.fill_point(&self.head, RED);
        for point in self.body.iter() {
            board.fill_point(point, RED);
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if dir == self.direction.get_opposite_dir() {
            return;
        }

        self.direction = dir;
        self.navigation_lock = true;
    }
}