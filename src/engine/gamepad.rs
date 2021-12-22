use crate::{wasm4, wasm4wrapper::*};

pub struct Gamepad {
    value: u8,
    previous_value: u8,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            value: 0,
            previous_value: 0,
        }
    }

    pub fn previous_gamepad(&self) -> Gamepad {
        self.previous_value.into()
    }

    pub fn update(&mut self) {
        let gamepad = get_gamepad(GamepadIndex::One);
        self.value = gamepad & (gamepad ^ self.previous_value);
        self.previous_value = gamepad;
    }

    pub fn released_left(&self) -> bool {
        !self.pressed_left() && self.previous_gamepad().pressed_left()
    }

    pub fn pressed_left(&self) -> bool {
        self.value & wasm4::BUTTON_LEFT != 0
    }

    pub fn pressed_right(&self) -> bool {
        self.value & wasm4::BUTTON_RIGHT != 0
    }

    pub fn released_right(&self) -> bool {
        !self.pressed_right() && self.previous_gamepad().pressed_right()
    }

    pub fn pressed_up(&self) -> bool {
        self.value & wasm4::BUTTON_UP != 0
    }

    pub fn released_up(&self) -> bool {
        !self.pressed_up() && self.previous_gamepad().pressed_up()
    }

    pub fn pressed_down(&self) -> bool {
        self.value & wasm4::BUTTON_DOWN != 0
    }

    pub fn released_down(&self) -> bool {
        !self.pressed_down() && self.previous_gamepad().pressed_down()
    }

    pub fn pressed_button_one(&self) -> bool {
        self.value & wasm4::BUTTON_1 != 0
    }

    pub fn released_button_one(&self) -> bool {
        !self.pressed_button_one() && self.previous_gamepad().pressed_button_one()
    }

    pub fn pressed_button_two(&self) -> bool {
        self.value & wasm4::BUTTON_2 != 0
    }

    pub fn released_button_two(&self) -> bool {
        !self.pressed_button_two() && self.previous_gamepad().pressed_button_two()
    }
}

impl Default for Gamepad {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for Gamepad {
    fn from(value: u8) -> Self {
        Gamepad {
            value,
            previous_value: 0,
        }
    }
}
