use crate::engine::math::rng::Rng;

// Pixel Sprite Generator v0.0.2
//
// This is a Rust version of the javascript version of David Bollinger's pixelrobots and
// pixelspaceships algorithm.
//
// Javascript license:
// The MIT License (MIT)
// Copyright (c) 2014 Zelimir Fedoran
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Javascript version:
// https://github.com/zfedoran/pixel-sprite-generator/
//
// More info:
// http://www.davebollinger.com/works/pixelrobots/
// http://www.davebollinger.com/works/pixelspaceships/
//
// Archived website (links above are down):
// http://web.archive.org/web/20080228054405/http://www.davebollinger.com/works/pixelrobots/
// http://web.archive.org/web/20080228054410/http://www.davebollinger.com/works/pixelspaceships/

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
pub enum MaskData {
    AlwaysBorder = -1,
    Empty = 0,
    EmptyBody = 1,
    BorderBody = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
pub enum SpriteData {
    Border = -1,
    Empty = 0,
    Body = 1,
}

impl From<i8> for MaskData {
    fn from(value: i8) -> Self {
        match value {
            -1 => MaskData::AlwaysBorder,
            0 => MaskData::Empty,
            1 => MaskData::EmptyBody,
            2 => MaskData::BorderBody,
            _ => MaskData::Empty,
        }
    }
}

impl From<SpriteData> for MaskData {
    fn from(sd: SpriteData) -> Self {
        match sd {
            SpriteData::Border => MaskData::AlwaysBorder,
            SpriteData::Empty => MaskData::Empty,
            SpriteData::Body => MaskData::EmptyBody,
        }
    }
}

impl From<MaskData> for SpriteData {
    fn from(md: MaskData) -> Self {
        match md {
            MaskData::AlwaysBorder => SpriteData::Border,
            MaskData::Empty => SpriteData::Empty,
            MaskData::EmptyBody => SpriteData::Border,
            _ => SpriteData::Empty,
        }
    }
}

pub struct Mask<const SIZE: usize> {
    pub width: usize,
    pub height: usize,
    pub data: [MaskData; SIZE],
    pub mirror_x: bool,
    pub mirror_y: bool,
}

impl<const SIZE: usize> Mask<SIZE> {
    pub fn new(
        width: usize,
        height: usize,
        data: [MaskData; SIZE],
        mirror_x: bool,
        mirror_y: bool,
    ) -> Self {
        assert_eq!(SIZE, width * height);
        Self {
            width,
            height,
            data,
            mirror_x,
            mirror_y,
        }
    }
}

pub struct PixelSprite<TRng: Rng + Copy, const SIZE: usize, const MASK_SIZE: usize> {
    width: usize,
    height: usize,
    mask: Mask<MASK_SIZE>,
    data: [SpriteData; SIZE],
    rng: TRng,
}

impl<TRNG: Rng + Copy, const SIZE: usize, const MASK_SIZE: usize>
    PixelSprite<TRNG, SIZE, MASK_SIZE>
{
    pub fn new(mask: Mask<MASK_SIZE>, rng: TRNG) -> Self {
        let mut sprite = Self {
            width: mask.width * if mask.mirror_x { 2 } else { 1 },
            height: mask.height * if mask.mirror_y { 2 } else { 1 },
            data: [SpriteData::Border; SIZE],
            mask,
            rng,
        };
        sprite.init();
        sprite
    }

    fn init(&mut self) {
        self.apply_mask();
        self.generate();
        if self.mask.mirror_x {
            self.mirror_x();
        }
        if self.mask.mirror_y {
            self.mirror_y();
        }
        self.generate_edges();
    }

    fn get_mask_data(&self, x: usize, y: usize) -> MaskData {
        self.mask.data[x + y * self.mask.width]
    }

    fn get_data(&self, x: usize, y: usize) -> SpriteData {
        self.data[x + y * self.width]
    }

    fn set_data(&mut self, x: usize, y: usize, data: SpriteData) {
        self.data[x + y * self.width] = data;
    }

    fn apply_mask(&mut self) {
        for y in 0..self.mask.height {
            for x in 0..self.mask.width {
                self.set_data(x, y, self.mask.data[y * self.mask.width + x].into())
            }
        }
    }

    fn generate(&mut self) {
        let w = self.mask.width;
        let h = self.mask.height;

        for y in 0..h {
            for x in 0..w {
                match self.get_mask_data(x, y) {
                    MaskData::EmptyBody => {
                        if self.rng.coin_flip() {
                            self.set_data(x, y, SpriteData::Body);
                        } else {
                            self.set_data(x, y, SpriteData::Empty);
                        }
                    }
                    MaskData::BorderBody => {
                        if self.rng.coin_flip() {
                            self.set_data(x, y, SpriteData::Body);
                        } else {
                            self.set_data(x, y, SpriteData::Border);
                        }
                    }
                    MaskData::AlwaysBorder => self.set_data(x, y, SpriteData::Border),
                    MaskData::Empty => self.set_data(x, y, SpriteData::Empty),
                }
            }
        }
    }

    fn generate_edges(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_data(x, y) == SpriteData::Body {
                    if y > 0 && self.get_data(x, y - 1) == SpriteData::Empty {
                        self.set_data(x, y - 1, SpriteData::Border);
                    }
                    if y + 1 < self.height && self.get_data(x, y + 1) == SpriteData::Empty {
                        self.set_data(x, y + 1, SpriteData::Border)
                    }
                    if x > 0 && self.get_data(x - 1, y) == SpriteData::Empty {
                        self.set_data(x - 1, y, SpriteData::Border);
                    }
                    if x + 1 < self.width && self.get_data(x + 1, y) == SpriteData::Empty {
                        self.set_data(x + 1, y, SpriteData::Border);
                    }
                }
            }
        }
    }

    fn mirror_x(&mut self) {
        for y in 0..self.height {
            for x in 0..self.mask.width {
                self.set_data(self.width - x - 1, y, self.get_data(x, y));
            }
        }
    }

    fn mirror_y(&mut self) {
        for y in 0..self.mask.height {
            for x in 0..self.mask.width {
                self.set_data(x, self.height - y - 1, self.get_data(x, y));
            }
        }
    }
}

// impl<TRNG: Rng + Copy, const SIZE: usize, const MASK_SIZE: usize> Display
//     for PixelSprite<TRNG, SIZE, MASK_SIZE>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut result = String::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 match self.get_data(x, y) {
//                     SpriteData::Border => result += "▓",
//                     SpriteData::Empty => result += "░",
//                     SpriteData::Body => result += "▒",
//                 }
//             }
//             result += "\n";
//         }
//         write!(f, "{}", result)
//     }
// }
