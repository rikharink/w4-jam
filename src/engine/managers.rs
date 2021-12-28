use super::{gamepad::Gamepad, mouse::Mouse};
use lazy_static::lazy_static;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::sync::Mutex;

lazy_static! {
    pub static ref MANAGERS: Mutex<Managers> = Mutex::new(Managers {
        rng: Pcg64::seed_from_u64(4),
        gamepad: Gamepad::new(),
        mouse: Mouse::new(),
    });
}

pub struct Managers {
    pub gamepad: Gamepad,
    pub mouse: Mouse,
    pub rng: Pcg64,
}

impl Managers {
    pub fn update(&mut self) {
        self.mouse.update();
        self.gamepad.update();
    }

    pub fn seed(&mut self, seed: u64) {
        self.rng = Pcg64::seed_from_u64(seed);
    }
}
