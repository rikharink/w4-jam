mod splitmix64;
mod xoshiro;
use std::ops::RangeInclusive;

pub use splitmix64::*;
pub use xoshiro::*;

pub trait Rng {
    fn max(&self) -> u64;

    fn next(&mut self) -> u64;

    fn next_f64(&mut self) -> f64 {
        let result = self.next();
        ((result >> 11) as f64) * 1.110223e-16
    }

    fn next_range(&mut self, range: RangeInclusive<u64>) -> u64 {
        let start = range.start();
        let end = range.end();
        start + self.next() / (self.max() / (end - start + 1) + 1)
    }

    fn seed(&mut self, seed: u64);
}
