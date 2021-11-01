use zoon::*;

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

fn root() -> impl IntoIterator<Item = impl Element> {
    element_vec![
        Button::new().label("-").on_press(decrement),
        Text::with_signal(counter().signal()),
        Button::new().label("+").on_press(increment),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
