use zoon::*;



fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    El::new().child("Tauri Web FS")
}
