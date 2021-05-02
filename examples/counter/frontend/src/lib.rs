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

// fn root() -> Column {
//     col![
//         button![button::on_press(decrement), "-"],
//         column::item_signal(counter().signal()),
//         button![button::on_press(increment), "+"],
//     ]
// }

fn root() -> Column {
    Column::new()
        .item(Button::new().on_press(decrement).label("-"))
        .item_signal(counter().signal())
        .item(Button::new().on_press(increment).label("+"))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
