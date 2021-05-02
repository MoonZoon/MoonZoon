use zoon::*;
use zoon::futures_signals::signal::Mutable;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

fn root() -> impl Element {
    Column::new()
        .item(Button::new().on_press(decrement).label("-"))
        // .item(Text::with_signal(counter().signal()))
        .item_signal(counter().signal())
        .item(Button::new().on_press(increment).label("+"))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
