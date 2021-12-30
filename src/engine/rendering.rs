use crate::wasm4::{self, FRAMEBUFFER};

pub fn set_draw_colors(draw_colors: &DrawColor) {
    unsafe {
        *wasm4::DRAW_COLORS = draw_colors.value();
    }
}

pub fn get_draw_colors() -> u16 {
    unsafe { *wasm4::DRAW_COLORS }
}

#[derive(Debug, Copy, Clone)]
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

pub struct DrawColor {
    colors: [u16; 4],
}

impl DrawColor {
    pub fn new(color_1: u16, color_2: u16, color_3: u16, color_4: u16) -> Self {
        Self {
            colors: [color_1, color_2, color_3, color_4],
        }
    }

    pub fn color_1(&self) -> u16 {
        self.colors[0]
    }

    pub fn color_2(&self) -> u16 {
        self.colors[1]
    }

    pub fn color_3(&self) -> u16 {
        self.colors[2]
    }

    pub fn color_4(&self) -> u16 {
        self.colors[3]
    }

    pub fn value(&self) -> u16 {
        (self.color_4() << 12) + (self.color_3() << 8) + (self.color_2() << 4) + self.color_1()
    }
}
