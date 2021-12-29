use super::vector::{IVec2, UVec2};

pub struct Rect {
    pub position: IVec2,
    pub size: UVec2,
}

impl Rect {
    pub(crate) fn new(position: IVec2, size: UVec2) -> Self {
        Self { position, size }
    }
}
