use crate::{
    engine::{
        imwui,
        managers::get_managers_mut,
        math::{rng::Rng, vector::IVec2},
        rendering::{DrawColor, Palette},
    },
    unwrap_abort,
};
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
        Self::default()
    }

    pub fn update(&mut self) {
        let managers = unwrap_abort(get_managers_mut().as_mut());
        self.current_frame = self.current_frame.overflowing_add(1).0;
        managers.update(self.current_frame);
        if imwui::button("start", IVec2(60, 70), &DrawColor::new(3, 4, 0, 0)) {
            managers.rng.seed(self.current_frame);
        }
    }
}
