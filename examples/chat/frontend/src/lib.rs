use zoon::*;

mod app;
mod markup;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", app::view::root);
    app::connection();
}
