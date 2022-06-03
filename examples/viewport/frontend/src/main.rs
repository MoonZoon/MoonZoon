use std::iter;
use zoon::{named_color::*, *};

// ------ ------
//   States
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

fn jump_to_top() {
    viewport_y().set(0);
}

fn jump_to_bottom() {
    viewport_y().set(i32::MAX);
}

// ------ ------
//   Handlers
// ------ ------

fn on_viewport_change(_scene: Scene, viewport: Viewport) {
    viewport_x().set(viewport.x());
    viewport_y().set(viewport.y());
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .s(Padding::all(20))
        .item(rectangles())
        .item(viewport_info())
        .item(jump_to_top_button())
        .item(jump_to_bottom_button())
}

// -- rectangles --

fn rectangles() -> impl Element {
    Column::new()
        .s(Width::exact(150))
        .s(Height::exact(200))
        .s(Spacing::new(20))
        .s(Padding::all(15))
        .s(Background::new().color(GRAY_8))
        .s(Scrollbars::both())
        .on_viewport_location_change(on_viewport_change)
        .viewport_x_signal(viewport_x().signal())
        .viewport_y_signal(viewport_y().signal())
        .items(iter::repeat_with(rectangle).take(5))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::exact(150))
        .s(Height::exact(50))
        .s(Background::new().color(RED_8))
}

// -- viewport_info --

fn viewport_info() -> impl Element {
    Column::new()
        .item(viewport_info_row("Viewport X:", viewport_x().signal()))
        .item(viewport_info_row("Viewport Y:", viewport_y().signal()))
}

fn viewport_info_row(
    label: &str,
    value: impl Signal<Item = i32> + Unpin + 'static,
) -> impl Element {
    Row::new()
        .s(Spacing::new(12))
        .item(El::new().child(label))
        .item(Text::with_signal(value))
}

// -- jump_button --

fn jump_to_top_button() -> impl Element {
    jump_button("Jump to Top", jump_to_top)
}

fn jump_to_bottom_button() -> impl Element {
    jump_button("Jump to Bottom", jump_to_bottom)
}

fn jump_button(label: &str, on_press: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
