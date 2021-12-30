use crate::engine::{managers::MANAGERS, point::Point, rendering::set_draw_colors};
use crate::wasm4;

pub fn begin_button(label: &str, position: Point, width: u32, height: u32) -> bool {
    let mouse = &MANAGERS.lock().expect("game_managers").mouse;
    let mouse_hit = mouse.position.x >= position.x
        && mouse.position.y >= position.y
        && mouse.position.x <= position.x + (width as i32)
        && mouse.position.y <= position.y + (height as i32);
    set_draw_colors(0x43u16);
    if mouse_hit && mouse.left_pressed() {
        set_draw_colors(0x34u16);
    }
    wasm4::rect(position.x, position.y, width, height);

    set_draw_colors(0x34u16);
    if mouse_hit && mouse.left_pressed() {
        set_draw_colors(0x43u16);
    }
    wasm4::text(label, position.x + 6, position.y + 7);
    mouse.left_released() && mouse_hit
}
