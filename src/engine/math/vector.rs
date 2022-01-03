use core::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UVec2(pub u32, pub u32);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IVec2(pub i32, pub i32);

impl IVec2 {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

impl UVec2 {
    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }

    pub fn width(&self) -> u32 {
        self.x()
    }

    pub fn height(&self) -> u32 {
        self.y()
    }
}

impl Add<UVec2> for UVec2 {
    type Output = Self;

    fn add(self, rhs: UVec2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<UVec2> for UVec2 {
    type Output = Self;

    fn sub(self, rhs: UVec2) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
