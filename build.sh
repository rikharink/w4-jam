#!/bin/bash
cargo build --release
wasm-opt ./target/wasm32-unknown-unknown/release/cart.wasm -o ./target/wasm32-unknown-unknown/release/cart.wasm -Oz --zero-filled-memory --strip-producers --strip-debug --strip-dwarf