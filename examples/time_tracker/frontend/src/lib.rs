use zoon::*;

mod app;
// mod login;
// mod clients_and_projects;
// mod time_tracker;
// mod time_blocks;
// mod home;
mod router;

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::view::root);
    // connection();
}
