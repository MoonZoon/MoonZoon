use zoon::*;

fn main() {
    start_app("app", root);
}

#[derive(Clone, Copy)]
struct DecreaseButtonPressed;
#[derive(Clone, Copy)]
struct IncreaseButtonPressed;

static COUNTER: Lazy<Mutable<i32>> = Lazy::new(|| {
    on(|DecreaseButtonPressed| *COUNTER.lock_mut() -= 1);
    on(|IncreaseButtonPressed| *COUNTER.lock_mut() += 1);
    Mutable::new(0)
});

fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Gap::new().x(15))
        .item(counter_button("-", || emit(DecreaseButtonPressed)))
        .item_signal(COUNTER.signal())
        .item(counter_button("+", || emit(IncreaseButtonPressed)))
}

fn counter_button(label: &str, on_press: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(45))
        .s(RoundedCorners::all_max())
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| hsluv!(300, 75, 85), || hsluv!(300, 75, 75))))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}
