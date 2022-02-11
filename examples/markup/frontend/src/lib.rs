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
    let el = RawHtmlEl::from_markup(r#"
        <div>
            <button id="btn-decrement">-</button>
            <p id="counter-value">123</p>
            <button id="btn-increment">+</button>
        </div>
    "#).unwrap_throw();

    let mut btn_decrement = el.find_html_child("#btn-decrement").unwrap_throw();
    btn_decrement = btn_decrement.event_handler(|_: events::Click| decrement());

    let mut counter_value = el.find_html_child("#counter-value").unwrap_throw();
    counter_value = counter_value.child_signal(counter().signal());

    let mut btn_increment = el.find_html_child("#btn-increment").unwrap_throw();
    btn_increment = btn_increment.event_handler(|_: events::Click| increment());

    el.after_remove(move |_| {
        drop(btn_decrement);
        drop(counter_value);
        drop(btn_increment);
    })
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
