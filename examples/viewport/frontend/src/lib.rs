use zoon::*;

fn root() -> impl Element {
    Text::new("viewport example")
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
