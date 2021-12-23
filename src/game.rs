use crate::engine::io::Saveable;
use crate::engine::{gamepad::Gamepad, mouse::Mouse};
use crate::snake::Snake;
use nanoserde::{DeBin, SerBin};

#[derive(SerBin, DeBin)]
pub struct Game {
    frame_count: usize,
    gamepad: Gamepad,
    mouse: Mouse,
    snake: Snake,
}

impl Saveable for Game {}

impl Default for Game {
    fn default() -> Self {
        Self {
            gamepad: Gamepad::new(),
            mouse: Mouse::new(),
            frame_count: 0,
            snake: Snake::new(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game::restore().unwrap_or_default()
    }

    fn handle_input(&mut self) {
        if self.gamepad.pressed_left() {
            self.snake.left();
        } else if self.gamepad.pressed_right() {
            self.snake.right();
        } else if self.gamepad.pressed_up() {
            self.snake.up();
        } else if self.gamepad.pressed_down() {
            self.snake.down();
        }
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.overflowing_add(1).0;
        self.gamepad.update();
        self.mouse.update();
        self.handle_input();
        let dropped_pos = self.snake.update();
        self.snake.draw();
        if self.frame_count % 60 == 0 {
            self.save();
        }
    }
}
