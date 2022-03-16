use zoon::{*, named_color::*};
use LayerId::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum LayerId {
    A,
    B,
    C,
}

#[static_ref]
fn layer_order() -> &'static Mutable<[LayerId; 3]> {
    Mutable::new([A, B, C])
}

fn bring_to_front(layer_id: LayerId) {
    let mut layers = layer_order().lock_mut();
    let layer_position = layers.iter().position(|id| id == &layer_id).unwrap_throw();
    layers.swap(0, layer_position);
}

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::new(180))
        .s(Height::new(180))
        .layer(rectangle(A, RED_6, Align::new()))
        .layer(rectangle(B, GREEN_6, Align::center()))
        .layer(rectangle(C, BLUE_6, Align::new().bottom().right()))
}

fn rectangle(layer_id: LayerId, color: HSLuv, align: Align) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .s(Width::new(100))
        .s(Height::new(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new().color_signal(hovered_signal.map_bool(
            move || color.update_l(|l| l + 5.), 
            move || color,
        )))
        .s(align)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || bring_to_front(layer_id))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
