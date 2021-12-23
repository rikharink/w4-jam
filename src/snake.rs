use nanoserde::{DeBin, SerBin};

use crate::engine::rendering::set_draw_color;
use crate::engine::{point, point::Point};
use crate::wasm4;

#[derive(SerBin, DeBin)]
pub struct Snake {
    pub direction: Point,
    pub body: Vec<Point>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![Point::new(2, 0), Point::new(1, 0), Point::new(0, 0)],
            direction: point::RIGHT,
        }
    }

    pub fn update(&mut self) -> Option<Point> {
        if !self.body.is_empty() {
            self.body.insert(
                0,
                Point {
                    x: (self.body[0].x + self.direction.x) % 20,
                    y: (self.body[0].y + self.direction.y) % 20,
                },
            );

            if self.body[0].x < 0 {
                self.body[0].x = 19;
            }

            if self.body[0].y < 0 {
                self.body[0].y = 19;
            }

            self.body.pop()
        } else {
            None
        }
    }

    pub fn draw(&self) {
        if let Some(first) = self.body.first() {
            set_draw_color(0x43u16);

            for &Point { x, y } in self.body.iter() {
                wasm4::rect(x * 8, y * 8, 8, 8);
            }

            set_draw_color(0x4u16);
            wasm4::rect(first.x * 8, first.y * 8, 8, 8);
        }
    }

    pub fn left(&mut self) {
        if self.direction.x == 0 {
            self.direction = point::LEFT;
        }
    }

    pub fn right(&mut self) {
        if self.direction.x == 0 {
            self.direction = point::RIGHT;
        }
    }

    pub fn up(&mut self) {
        if self.direction.y == 0 {
            self.direction = point::UP;
        }
    }

    pub fn down(&mut self) {
        if self.direction.y == 0 {
            self.direction = point::DOWN;
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}
