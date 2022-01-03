#!/bin/bash

set -euo pipefail
set -x

rm -rf dist
mkdir dist

cargo build --release
cp target/wasm32-unknown-unknown/release/*.wasm dist/cart.wasm

wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o dist/cart.wasm dist/cart.wasm
wasm-opt -Oz --strip-producers --strip-debug --dce --zero-filled-memory dist/cart.wasm -o dist/cart.wasm
stat -c%s dist/cart.wasm
