use educe::Educe;
use zoon::*;
use std::sync::Arc;

#[derive(Educe)]
#[educe(Default(new))]
struct Store {
    #[educe(Default(expression = r#"Mutable::new(Arc::new("Hello".to_owned()))"#))]
    text_a: Mutable<Arc<String>>,
    #[educe(Default(expression = r#"Mutable::new(Arc::new("World!".to_owned()))"#))]
    text_b: Mutable<Arc<String>>,
    joined_texts: Mutable<Arc<String>>,
}

#[static_ref]
fn store() -> &'static Store {
    Store::new()
}

fn main() {
    Task::start(
        map_ref! {
            let text_a = store().text_a.signal_cloned(),
            let text_b = store().text_b.signal_cloned() => {
                Task::start_blocking(clone!((text_a, text_b) move || {
                    let joined_texts = format!("{text_a} {text_b}");
                    store().joined_texts.set(joined_texts.into());
                }))
            }
        }.to_future()
    );
    start_app("app", root);
}

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20).top(150))
        .s(Align::new().center_x())
        .s(Gap::new().y(70))
        .item(field("Text A", store().text_a.clone(), false))
        .item(field("Text B", store().text_b.clone(), false))
        .item(field("Joined texts", store().joined_texts.clone(), true))
}

fn field(label: &str, text: Mutable<Arc<String>>, read_only: bool) -> impl Element {
    Column::new()
        .s(Gap::new().y(15))
        .item(Label::new().for_input(label).label(label))
        .item(
            TextArea::new()
                .id(label)
                .s(Width::exact(350))
                .s(Height::exact(50))
                .s(Align::new().center_x())
                .s(Outline::outer())
                .s(Padding::new().x(4).y(2))
                .s(Cursor::new(read_only.then(|| CursorIcon::Default)))
                .s(Background::new().color(read_only.then_some(hsluv!(0, 0, 95))))
                .update_raw_el(|raw_el| {
                    // @TODO Add to Zoon something like `TextArea::resizing(Resizing::Vertical)`
                    raw_el.style("resize", "vertical")
                })
                .read_only(read_only)
                // @TODO replace `(*text).clone()` with `text.unwrap_or_clone()` once stable
                // @TODO or/and impl `IntoCowStr` for `Arc` and `Rc` or is there a better solution?
                .text_signal(text.signal_cloned().map(|text| (*text).clone()))
                .on_change(move |new_text| text.set(new_text.into())),
        )
}
