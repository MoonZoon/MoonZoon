#![no_std]

use zoon::*;

mod app;
mod login;
mod clients_and_projects;
mod time_tracker;
mod time_blocks;
mod home;

#[wasm_bindgen(start)]
pub fn start() {
    start!(app)
}
