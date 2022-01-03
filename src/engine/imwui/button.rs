use crate::{
    engine::{
        managers::get_managers,
        math::{
            geometry::Rect,
            vector::{IVec2, UVec2},
        },
        rendering::{get_draw_colors, set_draw_colors, DrawColor},
    },
    wasm4,
};

use super::{control::Response, measure_text};

pub fn button(label: &str, position: IVec2) -> Response {
    let color = get_draw_colors();
    let inverse_color = DrawColor::new(
        color.color_2(),
        color.color_1(),
        color.color_3(),
        color.color_4(),
    );

    let padding: UVec2 = UVec2(4, 4);
    let size = (measure_text(label) + padding + padding) - UVec2(1, 1);
    let managers = get_managers().as_ref().unwrap();
    let mouse = &managers.mouse;

    let mouse_hit = mouse.hits(Rect::new(position, size));
    let pressed = mouse.left_pressed() & mouse_hit;
    if pressed {
        set_draw_colors(&inverse_color);
    }
    wasm4::rect(position.x(), position.y(), size.width(), size.height());
    set_draw_colors(&inverse_color);
    if pressed {
        set_draw_colors(&color);
    }
    wasm4::text(
        label,
        position.x() + padding.x() as i32,
        position.y() + padding.y() as i32,
    );
    set_draw_colors(&color);

    if pressed {
        Response::Pressed(position)
    } else if mouse.left_released() && mouse_hit {
        Response::Clicked(position)
    } else {
        Response::None
    }
}
