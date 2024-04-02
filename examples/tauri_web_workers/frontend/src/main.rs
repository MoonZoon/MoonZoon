use frontend::*;

fn main() {
    STORE.init_lazy();
    start_app("app", root);
}
