use crate::engine::{math::rng::Rng, rendering::put_pixel};

pub struct Background<T: Rng> {
    rng: T,
}

impl<T: Rng + Copy> Background<T> {
    pub fn new(number_of_stars: usize, color: u8) -> Self {
        Self { rng: T::new(123) }
    }

    pub fn regenerate(&mut self, number_of_stars: usize) {
        self.rng = T::new(self.rng.next());
    }

    pub fn render(&mut self) {
        let mut rng = self.rng;
        for i in 0..25600 {
            let x = i % 160;
            let y = i / 160;
            if rng.next_f64() >= 0.5 {
                put_pixel(x, y, 3);
            }
        }
        self.rng.next();
    }
}
