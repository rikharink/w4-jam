use crate::engine::{managers::get_managers_mut, rendering::Palette};

use super::scenes::{self, Scene};
static mut STATE: Option<State> = None;

pub fn init() {
    let state = unsafe { &mut STATE };
    *state = Some(State {
        current_frame: Default::default(),
        palette: Palette([0x46425e, 0x5b768d, 0xd17c7c, 0xf6c6a8]),
        current_scene: Scene::MainMenu,
    });
}

pub fn get_state() -> &'static Option<State> {
    unsafe { &STATE }
}

pub fn get_state_mut() -> &'static mut Option<State> {
    unsafe { &mut STATE }
}

pub struct State {
    pub current_frame: u64,
    pub palette: Palette,
    pub current_scene: Scene,
}

impl State {
    pub fn tick(&mut self) {
        self.current_frame = self.current_frame.overflowing_add(1).0;
        let managers = get_managers_mut().as_mut().unwrap();
        managers.update(self.current_frame);
        self.tick_current_scene();
    }

    fn tick_current_scene(&mut self) {
        match self.current_scene {
            Scene::MainMenu => scenes::main_menu::tick(),
            Scene::Game => scenes::game::tick(),
        }
    }
}
