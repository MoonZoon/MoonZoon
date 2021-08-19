use zoon::*;

// -- stopwatch --

#[static_ref]
fn seconds() -> &'static Mutable<u32> {
    Mutable::new(0)
}

#[static_ref]
fn stopwatch() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
}

fn stopwatch_enabled() -> impl Signal<Item = bool> {
    stopwatch().signal_ref(Option::is_some)
}

fn start_stopwatch() {
    seconds().take();
    stopwatch().set(Some(Timer::new(1_000, increment_seconds)));
}

fn increment_seconds() {
    seconds().update(|seconds| seconds + 1);
}

fn stop_stopwatch() {
    stopwatch().take();
}

// -- timeout --

#[static_ref]
fn timeout() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
}

fn timeout_enabled() -> impl Signal<Item = bool> {
    timeout().signal_ref(Option::is_some)
}

fn start_timeout() {
    timeout().set(Some(Timer::new(2_000, stop_timeout)));
}

fn stop_timeout() {
    timeout().take();
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Spacing::new(30))
        .item(stopwatch_panel())
        .item(timeout_panel())
}

fn stopwatch_panel() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item("Seconds: ")
        .item(Text::with_signal(seconds().signal()))
        .item_signal(stopwatch_enabled().map_bool(
            || stop_button(stop_stopwatch).left_either(),
            || start_button(start_stopwatch).right_either(),
        ))
}

fn timeout_panel() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item("2s Timeout")
        .item_signal(timeout_enabled().map_bool(
            || stop_button(stop_timeout).left_either(),
            || start_button(start_timeout).right_either(),
        ))
}

fn start_button(on_press: fn()) -> impl Element {
    button("Start", NamedColor::Green2, NamedColor::Green5, on_press)
}

fn stop_button(on_press: fn()) -> impl Element {
    button("Stop", NamedColor::Red2, NamedColor::Red5, on_press)
}

fn button(
    label: &str,
    bg_color: NamedColor,
    bg_color_hovered: NamedColor,
    on_press: fn(),
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(6))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(move || bg_color_hovered, move || bg_color)))
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
