pub use zoon::*;

mod store;
pub use store::*;

mod web_workers;
pub use web_workers::*;

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20).top(150))
        .s(Align::new().center_x())
        .s(Gap::new().y(80))
        .item(prime_panel())
        .item(markdown_panel())
}

fn prime_panel() -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Gap::new().x(20))
        .item(
            Button::new()
                .s(Padding::new().x(5).y(3))
                .s(Outline::outer())
                .label_signal(
                    store()
                        .is_generating_primes
                        .signal()
                        .map_bool(|| "Stop", || "Start"),
                )
                .on_press(|| store().is_generating_primes.update(not)),
        )
        .item(El::new().child_signal(store().prime.signal()))
}

fn markdown_panel() -> impl Element {
    Column::new()
        .s(Gap::new().y(20))
        .item(
            TextArea::new()
                .s(Width::exact(350))
                .s(Height::exact(100))
                .s(Align::new().center_x())
                .s(Outline::outer())
                .s(Padding::new().x(4).y(2))
                .label_hidden("markdown")
                .text_signal(store().markdown.signal_cloned())
                .on_change(|markdown| store().markdown.set(markdown)),
        )
        .item(
            RawHtmlEl::new("div")
                .class("markdown-body")
                .inner_markup_signal(store().html.signal_cloned()),
        )
}
