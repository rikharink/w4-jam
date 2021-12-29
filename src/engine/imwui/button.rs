use crate::{
    engine::{
        managers::MANAGERS,
        rendering::{set_draw_colors, DrawColor},
    },
    math::{
        geometry::Rect,
        vector::{IVec2, UVec2},
    },
    wasm4,
};

const FONT_SIZE: u32 = 8;

fn measure_text(text: &str) -> UVec2 {
    let lines: Vec<&str> = text.split('\n').collect();
    let nr_lines = lines.len() as u32;
    let max_width = lines.iter().map(|l| l.len()).max().unwrap() as u32;

    UVec2(max_width * FONT_SIZE, nr_lines * FONT_SIZE)
}

pub fn draw_button(label: &str, position: IVec2, color: &DrawColor) -> bool {
    let padding: UVec2 = UVec2(4, 4);
    let inverse_color = DrawColor::new(color.color_2(), color.color_1(), 0, 0);
    let size = (measure_text(label) + padding + padding) - UVec2(1, 1);
    let mouse = &MANAGERS.lock().expect("game_managers").mouse;

    let mouse_hit = mouse.hit(Rect::new(position, size));
    set_draw_colors(color);
    if mouse_hit && mouse.left_pressed() {
        set_draw_colors(&inverse_color);
    }
    wasm4::rect(position.x(), position.y(), size.width(), size.height());
    set_draw_colors(&inverse_color);
    if mouse_hit && mouse.left_pressed() {
        set_draw_colors(color);
    }
    wasm4::text(
        label,
        position.x() + padding.x() as i32,
        position.y() + padding.y() as i32,
    );
    mouse.left_released() && mouse_hit
}
