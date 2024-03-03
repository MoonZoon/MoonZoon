use std::iter;
use zoon::*;

static VIEWPORT_X: Lazy<Mutable<i32>> = Lazy::new(|| {
    on(|Viewport { x, .. }| VIEWPORT_X.set_neq(x));
    Mutable::new(0)
});
static VIEWPORT_Y: Lazy<Mutable<i32>> = Lazy::new(|| {
    on(|Viewport { y, .. }| VIEWPORT_Y.set_neq(y));
    on(|JumpToTop| VIEWPORT_Y.set_neq(0));
    on(|JumpToBottom| VIEWPORT_Y.set_neq(i32::MAX));
    Mutable::new(0)
});

#[derive(Clone, Copy)]
struct JumpToTop;
#[derive(Clone, Copy)]
struct JumpToBottom;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Width::fill().max(400))
        .s(Align::center())
        .s(Gap::both(20))
        .s(Padding::all(20))
        .item(rectangles())
        .item(viewport_info())
        .item(jump_button("Jump to Top", || emit(JumpToTop)))
        .item(jump_button("Jump to Bottom", || emit(JumpToBottom)))
}

// -- rectangles --

fn rectangles() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::exact(150))
        .s(Height::exact(200))
        .s(Gap::both(20))
        .s(Padding::all(15))
        .s(Background::new().color(color!("dimgray")))
        .s(Scrollbars::both())
        .on_viewport_location_change(|_, viewport| emit(viewport))
        .viewport_x_signal(VIEWPORT_X.signal())
        .viewport_y_signal(VIEWPORT_Y.signal())
        .items(iter::repeat_with(rectangle).take(5))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::exact(150))
        .s(Height::exact(50))
        .s(Background::new().color(color!("blue")))
}

// -- viewport_info --

fn viewport_info() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(viewport_info_row("Viewport X:", VIEWPORT_X.signal()))
        .item(viewport_info_row("Viewport Y:", VIEWPORT_Y.signal()))
}

fn viewport_info_row(
    label: &str,
    value: impl Signal<Item = i32> + Unpin + 'static,
) -> impl Element {
    Row::new()
        .s(Gap::both(12))
        .item(El::new().child(label))
        .item(Text::with_signal(value))
}

// -- jump_button --

fn jump_button(label: &str, on_press: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(5))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("green"), || color!("darkgreen"))))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}
