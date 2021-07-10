use zoon::*;
use std::iter;

fn root() -> impl Element {
    Column::new()
        .s(Width::new(150))
        .s(Height::new(200))
        .s(Spacing::new(20))
        .s(Padding::new().all(15))
        .s(Background::new().color(NamedColor::Gray5))
        .items(iter::repeat_with(rectangle).take(5))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::new(150))
        .s(Height::new(50))
        .s(Background::new().color(NamedColor::Red2))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
