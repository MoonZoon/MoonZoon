use zoon::*;

mod app;
mod login_page;
mod clients_and_projects_page;
mod time_tracker_page;
mod time_blocks_page;
mod home_page;
mod router;

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::view::root);
    // connection();
}
