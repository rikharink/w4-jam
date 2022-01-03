#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod wasm4;

use crate::engine::managers::get_managers;
use core::arch::wasm32;
use engine::{imwui, managers, rendering::set_palette};
use game::{get_state, get_state_mut};

#[no_mangle]
fn start() {
    game::init();
    imwui::init();
    managers::init();

    let managers = get_managers().as_ref().unwrap();
    let state = get_state().as_ref().unwrap();
    let palette = state.palette;
    set_palette(palette);
}

#[no_mangle]
fn update() {
    let state = get_state_mut().as_mut().unwrap();
    state.tick()
}

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo<'_>) -> ! {
    wasm4::trace("panic error");
    #[cfg(debug_assertions)]
    if let Some(cause) = _panic_info.payload().downcast_ref::<&str>() {
        wasm4::trace(cause);
    }
    wasm32::unreachable()
}
