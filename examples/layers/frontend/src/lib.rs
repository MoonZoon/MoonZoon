use zoon::{*, named_color::*};
use LayerId::*;

enum LayerId {
    A,
    B,
    C,
}

#[static_ref]
fn layer_order() -> &'static Mutable<[LayerId; 3]> {
    Mutable::new([A, B, C])
}

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::new(180))
        .s(Height::new(180))
        .layer(layer_a())
        .layer(layer_b())
        .layer(layer_c())
}

fn layer_a() -> impl Element {
    rectangle(A, RED_6, Align::new())
}

fn layer_b() -> impl Element {
    rectangle(B, GREEN_6, Align::center())
}

fn layer_c() -> impl Element {
    rectangle(C, BLUE_6, Align::new().bottom().right())
}

fn rectangle(layer_id: LayerId, color: HSLuv, align: Align) -> impl Element {
    El::new()
        .s(Width::new(100))
        .s(Height::new(100))
        .s(Background::new().color(color))
        .s(RoundedCorners::all(15))
        .s(Borders::all(Border::new()))
        .s(align)
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
