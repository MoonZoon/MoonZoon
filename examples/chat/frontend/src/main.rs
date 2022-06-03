use zoon::*;

mod app;
mod markup;

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", app::view::root);
    app::connection();
}
