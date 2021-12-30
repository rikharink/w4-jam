use super::vector::{IVec2, UVec2};

pub struct Rect {
    pub top_left: IVec2,
    pub size: UVec2,
}

impl Rect {
    pub(crate) fn new(position: IVec2, size: UVec2) -> Self {
        Self {
            top_left: position,
            size,
        }
    }
}
