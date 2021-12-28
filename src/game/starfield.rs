use crate::engine::{managers::MANAGERS, point::Point, rendering::put_pixel};
use rand::{distributions::Uniform, Rng};

pub struct Starfield {
    color: u8,
    stars: Vec<Point>,
}

impl Starfield {
    pub fn new(number_of_stars: usize, color: u8) -> Self {
        let rng = &mut MANAGERS.lock().expect("managers").rng;
        let grid_count = (number_of_stars as f32).sqrt().ceil() as i32;
        let grid_size = 160 / grid_count;

        let distribution = Uniform::new(1, grid_count);
        let mut stars: Vec<Point> = Vec::with_capacity(number_of_stars);
        for y in 0..grid_count {
            for x in 0..grid_count {
                let base_x = x * grid_size;
                let base_y = y * grid_size;
                stars.push(Point::new(
                    base_x + rng.sample(distribution),
                    base_y + rng.sample(distribution),
                ));
            }
        }
        Self { color, stars }
    }

    pub fn render(&self) {
        for star in &self.stars {
            put_pixel(star.x, star.y, self.color)
        }
    }
}
