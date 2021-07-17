use zoon::*;

mod app;
mod login;
mod report;
mod router;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::root);
}
