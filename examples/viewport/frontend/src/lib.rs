use zoon::*;
use std::array;

fn root() -> impl Element {
    Column::new()
        .s(Width::new(100))
        .s(Height::new(150))
        .s(Spacing::new(20))
        .s(Background::new().color(NamedColor::Gray5))
        .items(array::IntoIter::new([
            rectangle(),
            rectangle(),
            rectangle(),
            rectangle(),
            rectangle(),
        ]))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::new(100))
        .s(Height::new(50))
        .s(Background::new().color(NamedColor::Red2))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
