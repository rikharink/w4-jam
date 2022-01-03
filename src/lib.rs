#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod wasm4;
use crate::engine::managers::{get_managers, init_managers};
use engine::{managers::get_managers_mut, rendering::set_palette};

#[no_mangle]
fn start() {
    init_managers();
    let managers = get_managers().as_ref().unwrap();
    let palette = managers.state.palette;
    set_palette(palette);
}

#[no_mangle]
fn update() {
    let managers = get_managers_mut().as_mut().unwrap();
    managers.state.update()
}
