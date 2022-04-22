use zoon::*;

fn root() -> impl Element {
    Text::new("video!")
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
