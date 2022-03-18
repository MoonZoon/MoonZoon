use zoon::{named_color::*, *};
use LayerId::*;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, Eq)]
enum LayerId {
    A,
    B,
    C,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn layer_order() -> &'static Mutable<Vec<LayerId>> {
    Mutable::new(vec![A, B, C])
}

// ------ ------
//   Commands
// ------ ------

fn bring_to_front(layer_id: LayerId) {
    let mut layers = layer_order().lock_mut();
    let layer_position = layers.iter().position(|id| id == &layer_id).unwrap_throw();
    let layer = layers.remove(layer_position);
    layers.push(layer);
}

// ------ ------
//    Signals
// ------ ------

fn layer_index(layer_id: LayerId) -> impl Signal<Item = i32> {
    layer_order()
        .signal_ref(move |layers| {
            layers.iter().position(|id| id == &layer_id).unwrap_throw() as i32
        })
        .dedupe()
}

// ------ ------
//     View
// ------ ------

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
        .s(Background::new().color_signal(
            hovered_signal.map_bool(move || color.update_l(|l| l + 5.), move || color),
        ))
        .s(align)
        .s(LayerIndex::with_signal(layer_index(layer_id)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || bring_to_front(layer_id))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
