#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod snake;
mod wasm4;
use engine::rendering::set_palette;
use game::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    set_palette([0xfbf7f3, 0xe5b083, 0x416e5d, 0x20283d]);
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update()
}
