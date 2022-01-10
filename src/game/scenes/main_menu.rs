use crate::{
    engine::{imwui, managers::get_managers_mut, math::vector::IVec2},
    game::{get_state, get_state_mut},
};

use super::Scene;

pub fn tick() {
    let managers = get_managers_mut().as_mut().unwrap();
    let state = get_state().as_ref().unwrap();
    imwui::ui(&|ui| {
        ui.label("W4 JAM 2022", IVec2(42, 60));
        if ui.button("start", IVec2(60, 70)).clicked() {
            let state = get_state_mut().as_mut().unwrap();
            state.current_scene = Scene::Game;
        }
    });
}
