use crate::engine::{
    math::vector::IVec2,
    rendering::{get_draw_colors, set_draw_colors, DrawColor},
};

use super::{button, control::Response};

static mut IMWUI: Option<ImwuiBuilder> = None;

pub fn init() {
    let imwui = unsafe { &mut IMWUI };
    *imwui = Some(ImwuiBuilder::default());
}

pub struct ImwuiBuilder {
    pub button_color: DrawColor,
}

impl ImwuiBuilder {
    pub fn button(&mut self, label: &str, position: IVec2) -> Response {
        let color = get_draw_colors();
        set_draw_colors(&self.button_color);
        let response = button::button(label, position);
        set_draw_colors(&color);
        response
    }
}

impl Default for ImwuiBuilder {
    fn default() -> Self {
        Self {
            button_color: 0u16.into(),
        }
    }
}

pub fn ui(ui: &dyn Fn(&mut ImwuiBuilder)) {
    // Safety: wasm4 is single threaded
    ui(unsafe { IMWUI.as_mut().unwrap() });
}
