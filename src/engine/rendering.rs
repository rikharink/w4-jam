use nanoserde::{DeBin, SerBin};

use crate::wasm4::{self, FRAMEBUFFER};

pub fn set_draw_colors<T: Into<u16>>(draw_colors: T) {
    unsafe {
        *wasm4::DRAW_COLORS = draw_colors.into();
    }
}

pub fn get_draw_colors() -> u16 {
    unsafe { *wasm4::DRAW_COLORS }
}

#[derive(Debug, SerBin, DeBin, Copy, Clone)]
pub struct Palette(pub [u32; 4]);

pub fn set_palette(palette: Palette) {
    unsafe {
        *wasm4::PALETTE = palette.0;
    }
}

pub fn put_pixel(x: i32, y: i32, color: u8) {
    let idx = (y as usize * 160 + x as usize) >> 2;
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;
    unsafe {
        let framebuffer = FRAMEBUFFER.as_mut().expect("framebuffer ref");
        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}
