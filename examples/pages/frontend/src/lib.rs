use zoon::*;

mod app;
mod calc_page;
mod header;
mod login_page;
mod report_page;
mod router;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::root);
}
