use crate::{
    engine::{
        managers::get_managers,
        math::{geometry::Rect, vector::IVec2},
    },
    wasm4,
};

use super::{control::Response, measure_text};

pub fn label(text: &str, position: IVec2) -> Response {
    let size = measure_text(text);
    let managers = get_managers().as_ref().unwrap();
    wasm4::text(text, position.x(), position.y());

    let mouse = &managers.mouse;
    let mouse_hit = mouse.hits(Rect::new(position, size));
    if mouse.left_pressed() & mouse_hit {
        Response::Pressed(position)
    } else if mouse.left_released() && mouse_hit {
        Response::Clicked(position)
    } else {
        Response::None
    }
}
