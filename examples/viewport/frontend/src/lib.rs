use zoon::*;
use std::iter;

// ------ ------
//   Statics
// ------ ------

#[static_ref]
fn viewport_x() -> &'static Mutable<i32> {
    Mutable::new(0)
}

#[static_ref]
fn viewport_y() -> &'static Mutable<i32> {
    Mutable::new(0)
}

// ------ ------
//   Commands
// ------ ------

fn scroll_to_top() {
    viewport_y().set_neq(0);
}

fn scroll_to_bottom() {
    viewport_y().set_neq(i32::MAX);
}

// ------ ------
//   Handlers
// ------ ------

fn on_viewport_change(_scene: Scene, viewport: Viewport) {
    viewport_x().set_neq(viewport.x());
    viewport_y().set_neq(viewport.y());
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .s(Padding::new().all(20))
        .item(rectangles())
        .item(viewport_info())
        .item(scroll_to_top_button())
        .item(scroll_to_bottom_button())
}

fn rectangles() -> impl Element {
    Column::new()
        .s(Width::new(150))
        .s(Height::new(200))
        .s(Spacing::new(20))
        .s(Padding::new().all(15))
        .s(Background::new().color(NamedColor::Gray5))
        .on_viewport_location_change(on_viewport_change)
        .signal_for_viewport_x(viewport_x().signal())
        .signal_for_viewport_y(viewport_y().signal())
        .items(iter::repeat_with(rectangle).take(5))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::new(150))
        .s(Height::new(50))
        .s(Background::new().color(NamedColor::Red2))
}

fn viewport_info() -> impl Element {
    Column::new()
        .item(viewport_info_row("Viewport X:", viewport_x().signal()))
        .item(viewport_info_row("Viewport Y:", viewport_y().signal()))
}

fn viewport_info_row(label: &str, value: impl Signal<Item = i32> + Unpin + 'static) -> impl Element {
    Row::new()
        .s(Spacing::new(12))
        .item(El::new().child(label))
        .item(Text::with_signal(value))
}

fn scroll_to_top_button() -> impl Element {
    scroll_button("Scroll to Top", scroll_to_top)
}

fn scroll_to_bottom_button() -> impl Element {
    scroll_button("Scroll to Bottom", scroll_to_bottom)
}

fn scroll_button(label: &str, on_press: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().all(5))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2))
        )
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
