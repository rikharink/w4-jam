use crate::engine::{
    managers::MANAGERS,
    math::{rng::Rng, vector::UVec2},
    rendering::put_pixel,
};

pub struct Starfield {
    color: u8,
    stars: Vec<UVec2>,
}

impl Starfield {
    pub fn new(number_of_stars: usize, color: u8) -> Self {
        let rng = &mut MANAGERS.lock().expect("managers").rng;
        let grid_count = (number_of_stars as f32).sqrt().ceil() as u32;
        let grid_size = 160 / grid_count;
        let stars = get_stars(number_of_stars, grid_count, grid_size, rng);
        Self { color, stars }
    }

    pub fn regenerate(&mut self, number_of_stars: usize, color: u8) {
        let rng = &mut MANAGERS.lock().expect("managers").rng;
        let grid_count = (number_of_stars as f32).sqrt().ceil() as u32;
        let grid_size = 160 / grid_count;
        let stars = get_stars(number_of_stars, grid_count, grid_size, rng);
        self.color = color;
        self.stars = stars;
    }

    pub fn render(&self) {
        for star in &self.stars {
            put_pixel(star.x() as i32, star.y() as i32, self.color)
        }
    }
}

fn get_stars(
    number_of_stars: usize,
    grid_count: u32,
    grid_size: u32,
    rng: &mut crate::engine::math::rng::Xoshiro256PlusPlus,
) -> Vec<UVec2> {
    let mut stars: Vec<UVec2> = Vec::with_capacity(number_of_stars);
    for y in 0..grid_count {
        for x in 0..grid_count {
            let xrng: u32 = rng.next_range(0..=159) as u32;
            let yrng: u32 = rng.next_range(0..=159) as u32;
            stars.push(UVec2(xrng, yrng));
        }
    }
    stars
}
