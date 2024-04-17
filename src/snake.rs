use std::time::{Duration, Instant};

use macroquad::{color::RED, input::{get_last_key_pressed, KeyCode}};

use crate::board::{Board, Point};

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

struct Curve {
    point: Point,
    from_dir: Direction,
}

impl Curve {
    pub fn new(point: Point, from_dir: Direction) -> Self {
        Self {
            point,
            from_dir,
        }
    }
}

pub struct Snake {
    length: usize,
    head: Point,
    direction: Direction,
    curves: Vec<Curve>,
    last_move_time: Option<Instant>,
    speed: Duration,
    navigation_lock: bool,
}

impl Snake {
    pub fn new
    (board: &Board, speed: Duration) -> Self {
        Self {
            length: 5,
            head: board.get_center_point(),
            direction: Direction::Up,
            curves: vec![],
            last_move_time: None,
            speed,
            navigation_lock: false,
        }
    }

    pub fn handle_frame(&mut self) {
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
                self.head = self.direction.get_next_point(&self.head);
                self.last_move_time = Some(Instant::now());
                self.navigation_lock = false;
            }
        }
    }

    pub fn render(&self, board: &Board) {
        let mut current_point = self.head.clone();
        let mut current_dir = self.direction.clone();
        let mut processed_length = 0;

        while processed_length < self.length {
            board.fill_point(&current_point, RED);

            if let Some(curve) = self.curves.iter().find(|c| c.point.eq(&current_point)) {
                current_dir = curve.from_dir.clone();
            }

            current_point = current_dir.get_prev_point(&current_point);

            processed_length += 1;
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if dir == self.direction.get_opposite_dir() {
            return;
        }

        self.curves.push(Curve::new(self.head.clone(), self.direction.clone()));
        self.direction = dir;
        self.navigation_lock = true;
    }
}