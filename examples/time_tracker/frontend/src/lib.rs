use zoon::*;

mod app;
mod connection;
mod login_page;
mod clients_and_projects_page;
mod time_tracker_page;
mod time_blocks_page;
mod home_page;
mod router;
mod theme;

#[wasm_bindgen(start)]
pub fn start() {
    app::load_logged_user();
    theme::load_theme();
    start_app("app", app::root);
    connection::connection();
    router::router();
}
