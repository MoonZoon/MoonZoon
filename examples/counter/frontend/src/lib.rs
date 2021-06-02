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

fn root() -> impl Element {
    Column::new()
        .item(Button::new().label("-").on_press(decrement))
        .item(Text::with_signal(counter().signal()))
        .item(Button::new().label("+").on_press(increment))
}
// ------ Alternative ------

fn _root() -> impl Element {
    let counter = Mutable::new(0);
    let on_press = clone!((counter) move |step: i32| *counter.lock_mut() += step);
    Column::new()
        .item(Button::new().label("-").on_press(clone!((on_press) move || on_press(-1))))
        .item_signal(counter.signal())
        .item(Button::new().label("+").on_press(move || on_press(1)))
}

// ---------- // ----------- 

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}

