#![no_std]

use zoon::*;

mod app;

#[wasm_bindgen(start)]
pub fn start() {
    start!(app)
}
