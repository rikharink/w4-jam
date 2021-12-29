use crate::{
    math::{geometry::Rect, vector::IVec2},
    wasm4,
};
use nanoserde::{DeBin, SerBin};

#[derive(SerBin, DeBin)]
pub struct Mouse {
    pub previous_position: IVec2,
    pub position: IVec2,
    pub delta: IVec2,
    value: u8,
    previous_value: u8,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            previous_position: IVec2(0, 0),
            position: IVec2(0, 0),
            delta: IVec2(0, 0),
            value: 0,
            previous_value: 0,
        }
    }

    pub fn update(&mut self) {
        self.previous_value = self.value;
        self.previous_position = self.position;

        unsafe {
            self.position.0 = (*wasm4::MOUSE_X).into();
            self.position.1 = (*wasm4::MOUSE_Y).into();
            self.value = *wasm4::MOUSE_BUTTONS;
        }
        self.delta.0 = self.position.x() - self.previous_position.x();
        self.delta.1 = self.position.y() - self.previous_position.y();
    }

    pub fn left_pressed(&self) -> bool {
        self.value & wasm4::MOUSE_LEFT != 0
    }

    pub fn left_released(&self) -> bool {
        let prev: Mouse = self.previous_value.into();
        !self.left_pressed() && prev.left_pressed()
    }

    pub fn middle_pressed(&self) -> bool {
        self.value & wasm4::MOUSE_MIDDLE != 0
    }

    pub fn middle_released(&self) -> bool {
        let prev: Mouse = self.previous_value.into();
        !self.middle_pressed() && prev.middle_pressed()
    }

    pub fn right_pressed(&self) -> bool {
        self.value & wasm4::MOUSE_RIGHT != 0
    }

    pub fn right_released(&self) -> bool {
        let prev: Mouse = self.previous_value.into();
        !self.right_pressed() && prev.right_pressed()
    }

    pub fn hit(&self, hitbox: Rect) -> bool {
        self.position.x() >= hitbox.position.x()
            && self.position.y() >= hitbox.position.y()
            && self.position.x() <= hitbox.position.x() + (hitbox.size.width() as i32)
            && self.position.y() <= hitbox.position.y() + (hitbox.size.height() as i32)
    }
}

impl From<u8> for Mouse {
    fn from(value: u8) -> Self {
        let mut mouse = Mouse::new();
        mouse.value = value;
        mouse
    }
}
