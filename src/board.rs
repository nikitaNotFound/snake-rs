use macroquad::{color::{Color, BLACK, GRAY, YELLOW}, rand::{gen_range, rand}, shapes::draw_rectangle};

use crate::snake::Snake;

pub struct Board {
    size: f32,
    square_size: f32,
    square_count: i32,
    x_pad: f32,
    y_pad: f32,
    food: Point,
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn eq(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

impl Board {
    pub fn new(size: f32, square_count: i32, x_pad: f32, y_pad: f32) -> Self {
        Self {
            size,
            square_count,
            square_size: size / square_count as f32,
            x_pad,
            y_pad,
            food: Point::new(3, 5),
        }
    }

    pub fn restart(&mut self) {
        self.food = Point::new(3, 5);
    }

    pub fn get_square_size(&self) -> f32 { self.square_size }

    pub fn render(&self) {
        draw_rectangle(self.x_pad - 1.0, self.y_pad - 1.0, self.size + 2.0, self.size + 2.0, GRAY);
        draw_rectangle(self.x_pad, self.y_pad, 500.0, 500.0, BLACK);

        let mut x_index: i32 = 0;
        let mut y_index: i32 = 0;

        let x_point = self.x_pad;
        let y_point = self.y_pad;

        while y_index < self.square_count {
            while x_index < self.square_count {
                let x = x_point + self.square_size * x_index as f32;
                let y = y_point + self.square_size * y_index as f32;

                draw_rectangle(x, y, self.square_size, self.square_size, GRAY);
                draw_rectangle(x + 1.0, y + 1.0, self.square_size - 2.0, self.square_size - 2.0, BLACK);

                x_index += 1;
            }

            y_index += 1;
            x_index = 0;
        }

        self.fill_point(&self.food, YELLOW);
    }

    pub fn get_center_point(&self) -> Point {
        Point {
            x: (self.square_count as f32 / 2.0).ceil() as i32,
            y: (self.square_count as f32 / 2.0).ceil() as i32,
        }
    }

    pub fn check_point_overflow(&self, point: &Point) -> bool {
        if point.x > self.square_count {
            return true;
        } else if point.y > self.square_count {
            return true;
        } else if point.x == 0 {
            return true;
        } else if point.y == 0 {
            return true;
        }

        return false;
    }

    pub fn fill_point(&self, point: &Point, color: Color) {
        let x_point = self.x_pad + (point.x - 1) as f32 * self.square_size;
        let y_point = self.y_pad + (point.y - 1) as f32 * self.square_size;

        draw_rectangle(x_point + 1.0, y_point + 1.0, self.square_size - 2.0, self.square_size - 2.0, color);
    }

    pub fn spawn_food(&mut self, snake: &Snake) {
        loop {
            let point = Point::new(
                gen_range(1, self.square_count as i32),
                gen_range(1, self.square_count as i32),
            );

            if !snake.check_collision(&point) {
                self.food = point;
                return;
            }
        }
    }

    pub fn check_food_collision(&self, point: &Point) -> bool {
        point.eq(&self.food)
    }
}
