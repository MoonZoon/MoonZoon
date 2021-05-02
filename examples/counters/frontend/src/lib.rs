use zoon::*;

mod app;

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", app::view::root);
}
