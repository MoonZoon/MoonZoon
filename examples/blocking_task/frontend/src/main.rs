use std::sync::Arc;
use zoon::*;

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
    Task::start_blocking_with_tasks(
        |send_to_blocking| {
            map_ref! {
                let text_a = store().text_a.signal_cloned(),
                let text_b = store().text_b.signal_cloned() =>
                (text_a.clone(), text_b.clone())
            }
            .for_each_sync(send_to_blocking)
        },
        |from_input, _, send_to_output| {
            from_input.for_each_sync(move |(text_a, text_b)| {
                send_to_output(format!("{text_a} {text_b}"));
            })
        },
        |from_blocking| {
            from_blocking.for_each_sync(|joined_texts| {
                store().joined_texts.set(joined_texts.into());
            })
        },
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

fn field(label: &str, text: Mutable<Arc<String>>, is_output: bool) -> impl Element {
    Column::new()
        .s(Gap::new().y(15))
        .item(Label::new().for_input(label).label(label))
        .item(
            TextArea::new()
                .id(label)
                .s(Width::exact(350))
                .s(Height::exact(if is_output { 160 } else { 80 }))
                .s(Align::new().center_x())
                .s(Outline::outer())
                .s(Padding::new().x(4).y(2))
                .s(Cursor::new(is_output.then(|| CursorIcon::Default)))
                .s(Background::new().color(is_output.then_some(hsluv!(0, 0, 95))))
                .s(Resizable::y())
                .read_only(is_output)
                .text_signal(text.signal_cloned())
                .on_change(move |new_text| text.set(new_text.into())),
        )
}
