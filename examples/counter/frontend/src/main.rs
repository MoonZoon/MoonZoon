use zoon::*;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();

fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Gap::new().x(15))
        .item(counter_button("-", -1))
        .item_signal(COUNTER.signal())
        .item(counter_button("+", 1))
}

fn counter_button(label: &str, step: i32) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(45))
        .s(RoundedCorners::all_max())
        .s(Background::new()
            // color::hex("#124578")
            // palette::named::WHITE.into_format() ?
            // .color_signal_new(hovered_signal.map_bool(|| hsluv!(300, 75, 85), || hsluv!(300, 75, 75))))
            .color_signal_new(hovered_signal.map_bool(|| palette::Oklch::<f32>::new_const(0.5, 0.2, 85.0.into()), || palette::Oklch::new_const(0.8, 0.1, 75.0.into()))))
            // .color_signal_new(hovered_signal.map_bool(|| "#123569".parse::<palette::Srgb<u8>>().unwrap_throw().into_format(), || "#823569".parse::<palette::Srgb<u8>>().unwrap_throw().into_format())))
            // .color_signal_new(hovered_signal.map_bool(
            //     || palette::named::PINK.into_format(), 
            //     || palette::named::PALEVIOLETRED.into_format()
            // )))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(move || *COUNTER.lock_mut() += step)
}
