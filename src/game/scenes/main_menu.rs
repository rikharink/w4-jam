use crate::{
    engine::{imwui, math::vector::IVec2},
    game::get_state_mut,
};

use super::Scene;

pub fn tick() {
    imwui::ui(&|ui| {
        ui.label("W4 JAM 2022", IVec2(42, 60));
        if ui.button("start", IVec2(60, 70)).clicked() {
            let state = get_state_mut().as_mut().unwrap();
            state.pixel_sprite_factory.generate().print();
            state.current_scene = Scene::Game;
        }
    });
}
