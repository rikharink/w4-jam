use crate::engine::{
    math::vector::IVec2,
    rendering::{get_draw_colors, set_draw_colors},
};

use super::{button, control::Response, label, style::Style};

static mut IMWUI: Option<ImwuiBuilder> = None;

pub fn init() {
    let imwui = unsafe { &mut IMWUI };
    *imwui = Some(ImwuiBuilder::default());
}

#[derive(Default)]
pub struct ImwuiBuilder {
    pub style: Style,
}

impl ImwuiBuilder {
    pub fn button(&mut self, label: &str, position: IVec2) -> Response {
        let color = get_draw_colors();
        set_draw_colors(&self.style.button_color);
        let response = button::button(label, position);
        set_draw_colors(&color);
        response
    }

    pub fn label(&mut self, text: &str, position: IVec2) -> Response {
        let color = get_draw_colors();
        set_draw_colors(&self.style.label_color);
        let response = label::label(text, position);
        set_draw_colors(&color);
        response
    }
}

pub fn ui(ui: &dyn Fn(&mut ImwuiBuilder)) {
    // Safety: wasm4 is single threaded
    ui(unsafe { IMWUI.as_mut().unwrap() });
}

pub fn set_style(style: Style) {
    // Safety: wasm4 is single threaded
    let imwui = unsafe { IMWUI.as_mut().unwrap() };
    imwui.style = style;
}
