use zoon::*;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn markdown_to_html(markdown: &str) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(markdown, options);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    html_text
}


fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

fn root() -> impl Element {
    Column::new()
        .item(html_example())
        .item(markdown_example())
}

fn html_example() -> impl Element {
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

fn markdown_example() -> impl Element {
    RawHtmlEl::from_markup(
        markdown_to_html(include_str!("markdown_page.md"))
    ).unwrap_throw()
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
