use crate::{
    engine::{imwui, managers::get_managers_mut, math::vector::IVec2},
    game::get_state_mut,
};

use super::Scene;

pub fn tick() {
    let managers = get_managers_mut().as_mut().unwrap();

    imwui::ui(&|ui| {
        if ui.button("pause", IVec2(60, 70)).clicked() {
            let state = get_state_mut().as_mut().unwrap();
            state.current_scene = Scene::MainMenu;
        }
    });
}
