use std::sync::Mutex;

use super::background::Background;
use crate::engine::{
    imwui,
    managers::MANAGERS,
    math::{
        rng::{Rng, Xoshiro256Plus},
        vector::IVec2,
    },
    rendering::{DrawColor, Palette},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref BACKGROUND: Mutex<Background<Xoshiro256Plus>> = Mutex::new(Background::new(120, 3));
}

pub struct State {
    pub current_frame: u64,
    pub palette: Palette,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_frame: Default::default(),
            palette: Palette([0x46425e, 0x5b768d, 0xd17c7c, 0xf6c6a8]),
        }
    }
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn update(&mut self) {
        self.current_frame = self.current_frame.overflowing_add(1).0;
        BACKGROUND.lock().expect("background").render();
        MANAGERS
            .lock()
            .expect("managers")
            .update(self.current_frame);
        if imwui::button("start", IVec2(60, 70), &DrawColor::new(3, 4, 0, 0)) {
            MANAGERS
                .lock()
                .expect("managers")
                .rng
                .seed(self.current_frame);
            BACKGROUND.lock().expect("background").regenerate(120);
        }
    }
}
