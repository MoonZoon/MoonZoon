use zoon::*;
use zoon::futures_signals::signal::Mutable;
use std::ops::{AddAssign, SubAssign};

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().lock_mut().add_assign(1)
}

fn decrement() {
    counter().lock_mut().sub_assign(1)
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
