use crate::engine::math::vector::UVec2;

use super::{FONT_SIZE, MAX_TEXT_SIZE};

pub fn measure_text(text: &str) -> UVec2 {
    let lines: heapless::Vec<&str, MAX_TEXT_SIZE> = text.split('\n').collect();
    let nr_lines = lines.len() as u32;
    let max_width = lines.iter().map(|l| l.len()).max().unwrap() as u32;

    UVec2(max_width * FONT_SIZE, nr_lines * FONT_SIZE)
}
