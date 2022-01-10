use crate::engine::rendering::DrawColor;

#[derive(Clone)]
pub struct Style {
    pub label_color: DrawColor,
    pub button_color: DrawColor,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            label_color: DrawColor::new(3, 0, 0, 0),
            button_color: DrawColor::new(3, 4, 0, 0),
        }
    }
}
