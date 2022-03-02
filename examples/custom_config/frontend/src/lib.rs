use zoon::*;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn root() -> impl Element {
    Text::new("Custom Config")
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
