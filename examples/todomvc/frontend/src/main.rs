use zoon::*;

mod app;
mod router;

fn main() {
    app::load_todos();
    router::router();
    start_app("app", app::view::root);
}
