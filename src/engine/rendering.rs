use crate::wasm4;

pub fn set_draw_color<T: Into<u16>>(draw_colors: T) {
    unsafe {
        *wasm4::DRAW_COLORS = draw_colors.into();
    }
}

pub type Palette = [u32; 4];
pub fn set_palette(palette: Palette) {
    unsafe {
        *wasm4::PALETTE = palette;
    }
}