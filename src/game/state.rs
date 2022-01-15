use crate::engine::{
    imwui::Style,
    managers::get_managers_mut,
    math::rng::Xoshiro256PlusPlus,
    procgen::pixelsprite::{Mask, MaskData, PixelSpriteFactory},
    rendering::Palette,
};

use super::scenes::{self, Scene};
static mut STATE: Option<State> = None;
const SIZE: usize = 16 * 16;
const MASK_SIZE: usize = 8 * 8;

pub fn init() {
    let managers = get_managers_mut().as_mut().unwrap();
    let mask_data: [MaskData; MASK_SIZE] = [
        MaskData::Empty,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::EmptyBody,
        MaskData::BorderBody,
        MaskData::BorderBody,
        MaskData::AlwaysBorder,
        MaskData::Empty,
        MaskData::Empty,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
        MaskData::AlwaysBorder,
    ];
    let mask = Mask::new(8, 8, mask_data, true, true);
    let pixel_sprite_factory = PixelSpriteFactory::new(mask, managers.rng);
    let style = Style::default();
    let state = unsafe { &mut STATE };
    *state = Some(State {
        current_frame: Default::default(),
        palette: Palette([0x46425e, 0x5b768d, 0xd17c7c, 0xf6c6a8]),
        current_scene: Scene::MainMenu,
        pixel_sprite_factory,
        style,
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
    pub style: Style,
    pub pixel_sprite_factory: PixelSpriteFactory<Xoshiro256PlusPlus, SIZE, MASK_SIZE>,
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
