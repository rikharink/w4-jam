use super::starfield::Starfield;
use crate::{
    engine::{imwui, io::Saveable, managers::MANAGERS, point::Point, rendering::Palette},
    wasm4,
};
use lazy_static::lazy_static;
use nanoserde::{DeBin, SerBin};

lazy_static! {
    static ref BACKGROUND: Starfield = Starfield::new(120, 3);
}

#[derive(SerBin, DeBin)]
pub struct State {
    pub frame_count: u64,
    pub palette: Palette,
}

impl Saveable for State {}

impl Default for State {
    fn default() -> Self {
        Self {
            frame_count: Default::default(),
            palette: Palette([0x46425e, 0x5b768d, 0xd17c7c, 0xf6c6a8]),
        }
    }
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.overflowing_add(1).0;
        BACKGROUND.render();
        MANAGERS.lock().expect("managers").update();
        self.render();
    }

    pub fn render(&self) {
        self.save();
        Self::restore().expect("state restore");
        if imwui::begin_button("START", Point::new(60, 70), 50, 20) {
            wasm4::trace("CLICK START");
        }
    }
}
