#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod engine;
mod game;
mod wasm4;
use crate::engine::managers::{get_managers, init_managers};
use engine::{managers::get_managers_mut, rendering::set_palette};

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[no_mangle]
fn start() {
    init_managers();
    let managers = unwrap_abort(get_managers().as_ref());
    let palette = managers.state.palette;
    set_palette(palette);
}

#[no_mangle]
fn update() {
    let managers = unwrap_abort(get_managers_mut().as_mut());
    managers.state.update()
}
