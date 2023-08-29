use zoon::*;

fn main() {
    start_app("app", root);
}

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Gap::new().x(15))
        .item(counter_button("-", -1))
        .item_signal(counter().signal())
        .item(counter_button("+", 1))
}

fn counter_button(label: &str, step: i32) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(45))
        .s(Height::exact(25))
        .s(RoundedCorners::all_max())
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| hsluv!(300, 75, 85), || hsluv!(300, 75, 75))))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(move || *counter().lock_mut() += step)
}
