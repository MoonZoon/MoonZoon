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
        .s(Spacing::new(30))
        .item(stopwatch_panel())
        .item(timeout_panel())
}

fn stopwatch_panel() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(Text::with_signal(
            seconds()
                .signal()
                .map(|seconds| format!("Seconds: {}", seconds)),
        ))
        .item_signal(stopwatch_enabled().map_bool(
            || Button::new().label("Stop").on_press(stop_stopwatch),
            || Button::new().label("Start").on_press(start_stopwatch),
        ))
}

fn timeout_panel() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item("2s Timeout")
        .item_signal(timeout_enabled().map_bool(
            || Button::new().label("Stop").on_press(stop_timeout),
            || Button::new().label("Start").on_press(start_timeout),
        ))
}

fn start_button() -> impl Element {
    
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
