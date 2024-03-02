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
            .color_signal(hovered_signal.map_bool(|| color!("#edc8f5"), || color!("#E1A3EE", 0.8))))
        .s(Borders::all(
            Border::new()
                .width(2)
                .color(color!("oklch(0.6 0.182 350.53 / .7")),
        ))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}
