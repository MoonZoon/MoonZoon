use zoon::*;

mod app;

fn main() {
    start_app("app", app::view::root);
}
