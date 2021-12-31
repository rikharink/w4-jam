#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod wasm4;
use crate::{
    engine::{
        math::rng::Rng,
        procgen::pixelsprite::{Mask, MaskData},
    },
    game::State,
};
use engine::{
    math::rng::Xoshiro256Plus, procgen::pixelsprite::PixelSprite, rendering::set_palette,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[no_mangle]
fn start() {
    let palette = STATE.lock().expect("game_state").palette;
    set_palette(palette);
    const SPRITE_WIDTH: usize = 8;
    const SPRITE_HEIGHT: usize = 8;
    const SPRITE_SIZE: usize = SPRITE_WIDTH * SPRITE_HEIGHT;
    const MASK_WIDTH: usize = 4;
    const MASK_HEIGHT: usize = 4;
    const MASK_SIZE: usize = MASK_WIDTH * MASK_HEIGHT;
    let mask_data = [MaskData::Empty; MASK_SIZE];
    let mask = Mask::<MASK_SIZE>::new(MASK_WIDTH, MASK_HEIGHT, mask_data, true, true);
    let sprite =
        PixelSprite::<Xoshiro256Plus, SPRITE_SIZE, MASK_SIZE>::new(mask, Xoshiro256Plus::new(1283));
    wasm4::trace(sprite.to_string());
}

#[no_mangle]
fn update() {
    STATE.lock().expect("game_state").update()
}
