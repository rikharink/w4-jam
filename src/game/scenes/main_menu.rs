use crate::{
    engine::{imwui, math::{vector::IVec2, rng::Rng}, rendering::{DrawColor, put_pixel}, managers::get_managers_mut},
    game::get_state_mut,
};

use super::Scene;

pub fn tick() {
    let managers = get_managers_mut().as_mut().unwrap();
    let mut rng = managers.rng;
    for i in 0..25600 {
        let x = i % 160;
        let y = i / 160;
        if rng.next_f64() >= 0.5 {
            put_pixel(x, y, 3);
        }
    }

    imwui::ui(&|ui| {
        ui.button_color = DrawColor::new(3, 4, 0, 0);
        if ui.button("start", IVec2(60, 70)).clicked() {
            let state = get_state_mut().as_mut().unwrap();
            state.current_scene = Scene::Game;
        }
    });
}
