use crate::engine::{gamepad::Gamepad, mouse::Mouse};
use crate::snake::Snake;
use crate::wasm4;

pub struct Game {
    frame_count: usize,
    gamepad: Gamepad,
    mouse: Mouse,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            gamepad: Gamepad::new(),
            mouse: Mouse::new(),
            frame_count: 0,
            snake: Snake::new(),
        }
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
        if self.mouse.left_released() {
            wasm4::trace("mouse left released!");
        }
    }
}
