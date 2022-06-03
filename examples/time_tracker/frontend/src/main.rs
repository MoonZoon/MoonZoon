use zoon::*;

mod app;
mod clients_and_projects_page;
mod connection;
mod home_page;
mod login_page;
mod router;
mod theme;
mod time_blocks_page;
mod time_tracker_page;

fn main() {
    app::load_logged_user();
    theme::load_theme();
    start_app("app", app::root);
    connection::connection();
    router::router();
}
