use macroquad::{color::{Color, BLACK, GRAY}, shapes::draw_rectangle};

pub struct Board {
    size: f32,
    square_size: f32,
    square_count: isize,
    x_pad: f32,
    y_pad: f32,
}

#[derive(Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }

    pub fn eq(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

impl Board {
    pub fn new(size: f32, square_count: isize, x_pad: f32, y_pad: f32) -> Self {
        Self {
            size,
            square_count,
            square_size: size / square_count as f32,
            x_pad,
            y_pad
        }
    }

    pub fn get_square_size(&self) -> f32 { self.square_size }

    pub fn render(&self) {
        draw_rectangle(self.x_pad - 1.0, self.y_pad - 1.0, self.size + 2.0, self.size + 2.0, GRAY);
        draw_rectangle(self.x_pad, self.y_pad, 500.0, 500.0, BLACK);

        let mut x_index: isize = 0;
        let mut y_index: isize = 0;

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
    }

    pub fn get_center_point(&self) -> Point {
        Point {
            x: (self.square_count as f32 / 2.0).ceil() as isize,
            y: (self.square_count as f32 / 2.0).ceil() as isize,
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
}
