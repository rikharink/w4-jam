use crate::engine::math::vector::IVec2;

pub enum Response {
    Focused,
    Hovered(IVec2),
    Pressed(IVec2),
    Clicked(IVec2),
    DoubleClicked(IVec2),
    None,
}

impl Response {
    pub fn focused(&self) -> bool {
        matches!(self, Response::Focused)
    }

    pub fn hovered(&self) -> bool {
        matches!(self, Response::Hovered(_))
    }

    pub fn double_clicked(&self) -> bool {
        matches!(self, Response::DoubleClicked(_))
    }

    pub fn clicked(&self) -> bool {
        matches!(self, Response::Clicked(_))
    }

    pub fn pressed(&self) -> bool {
        matches!(self, Response::Hovered(_))
    }
}
