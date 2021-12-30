use super::{
    control::{gamepad::Gamepad, mouse::Mouse},
    math::rng::{Rng, Xoshiro256PlusPlus},
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref MANAGERS: Mutex<Managers<Xoshiro256PlusPlus>> = Mutex::new(Managers {
        rng: Xoshiro256PlusPlus::new(428281),
        gamepad: Gamepad::new(),
        mouse: Mouse::new(),
    });
}

pub struct Managers<T: Rng> {
    pub gamepad: Gamepad,
    pub mouse: Mouse,
    pub rng: T,
}

impl<T: Rng> Managers<T> {
    pub fn update(&mut self, current_frame: u64) {
        self.mouse.update(current_frame);
        self.gamepad.update();
    }
}
