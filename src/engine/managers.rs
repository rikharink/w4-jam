use super::{
    control::{gamepad::Gamepad, mouse::Mouse},
    math::rng::{Rng, Xoshiro256PlusPlus},
};

static mut MANAGERS: Option<Managers<Xoshiro256PlusPlus>> = None;

pub fn init() {
    let managers = unsafe { &mut MANAGERS };
    let mngs = Managers {
        rng: Xoshiro256PlusPlus::new(428281),
        gamepad: Gamepad::new(),
        mouse: Mouse::new(),
    };
    *managers = Some(mngs);
}

pub fn get_managers() -> &'static Option<Managers<Xoshiro256PlusPlus>> {
    //Safety: wasm4 is single threaded
    unsafe { &MANAGERS }
}

pub fn get_managers_mut() -> &'static mut Option<Managers<Xoshiro256PlusPlus>> {
    //Safety: wasm4 is single threaded
    unsafe { &mut MANAGERS }
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
