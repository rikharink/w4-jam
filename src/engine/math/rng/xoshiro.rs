use super::{Rng, SplitMix64};

/*
   Adapted from https://prng.di.unimi.it/xoshiro256plusplus.c
   Written in 2019 by David Blackman and Sebastiano Vigna (vigna@acm.org)
*/

#[derive(Clone)]
pub struct Xoshiro256PlusPlus {
    pub state: [u64; 4],
}

#[inline]
fn rotl(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

impl Rng for Xoshiro256PlusPlus {
    fn new(seed: u64) -> Self {
        let mut state = [0u64; 4];
        let mut seed_eng = SplitMix64::new(seed);
        state[0] = seed_eng.next();
        state[1] = seed_eng.next();
        state[2] = seed_eng.next();
        state[3] = seed_eng.next();
        Self { state }
    }

    fn max(&self) -> u64 {
        u64::MAX
    }

    fn next(&mut self) -> u64 {
        let s = &mut self.state;
        let result = rotl(s[0].wrapping_add(s[3]), 23).wrapping_add(s[0]);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = rotl(s[3], 45);
        result
    }

    fn seed(&mut self, seed: u64) {
        let mut state = self.state;
        let mut seed_eng = SplitMix64::new(seed);
        state[0] = seed_eng.next();
        state[1] = seed_eng.next();
        state[2] = seed_eng.next();
        state[3] = seed_eng.next();
    }
}

#[derive(Clone)]
pub struct Xoshiro256Plus {
    pub state: [u64; 4],
}

impl Rng for Xoshiro256Plus {
    fn new(seed: u64) -> Self {
        let mut state = [0u64; 4];
        let mut seed_eng = SplitMix64::new(seed);
        state[0] = seed_eng.next();
        state[1] = seed_eng.next();
        state[2] = seed_eng.next();
        state[3] = seed_eng.next();
        Self { state }
    }

    fn max(&self) -> u64 {
        u64::MAX
    }

    fn next(&mut self) -> u64 {
        let s = &mut self.state;
        let result = s[0].wrapping_add(s[3]);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = rotl(s[3], 45);
        result
    }

    fn seed(&mut self, seed: u64) {
        let mut state = self.state;
        let mut seed_eng = SplitMix64::new(seed);
        state[0] = seed_eng.next();
        state[1] = seed_eng.next();
        state[2] = seed_eng.next();
        state[3] = seed_eng.next();
    }
}
