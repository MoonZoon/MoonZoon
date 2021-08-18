use zoon::*;

mod app;
mod router;

#[wasm_bindgen(start)]
pub fn start() {
    app::load_todos();
    router::router();
    start_app("app", app::view::root);
}
