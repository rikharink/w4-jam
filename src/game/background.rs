use crate::engine::{
    managers::MANAGERS,
    math::rng::{Rng, Xoshiro256PlusPlus},
    rendering::put_pixel,
};

pub struct Background {
    rng: Xoshiro256PlusPlus,
}

impl Background {
    pub fn new(number_of_stars: usize, color: u8) -> Self {
        let rng = &mut MANAGERS.lock().expect("managers").rng;
        Self { rng: *rng }
    }

    pub fn regenerate(&mut self, number_of_stars: usize) {
        self.rng = Xoshiro256PlusPlus::new(self.rng.next());
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
