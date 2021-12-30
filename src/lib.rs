#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod wasm4;
use crate::game::State;
use engine::rendering::set_palette;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[no_mangle]
fn start() {
    let palette = STATE.lock().expect("game_state").palette;
    set_palette(palette);
}

#[no_mangle]
fn update() {
    STATE.lock().expect("game_state").update()
}
