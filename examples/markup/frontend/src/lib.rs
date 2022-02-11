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
    RawHtmlEl::from_markup(
        r#"
        <div>
            <button id="btn-decrement">-</button>
            <p id="counter-value"></p>
            <button id="btn-increment">+</button>
        </div>
        "#,
    )
    .unwrap_throw()
    .update_html_child("#btn-decrement", |child| {
        child.event_handler(|_: events::Click| decrement())
    })
    .update_html_child("#counter-value", |child| {
        child.child_signal(counter().signal())
    })
    .update_html_child("#btn-increment", |child| {
        child.event_handler(|_: events::Click| increment())
    })
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
