use zoon::*;
use zoon::futures_signals::signal::Mutable;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().replace_with(|counter| *counter + 1);
}

fn decrement() {
    counter().replace_with(|counter| *counter - 1);
}

fn root() -> Column {
    col![
        button![button::on_press(decrement), "-"],
        column::item_signal(counter().signal()),
        button![button::on_press(increment), "+"],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
