use zoon::*;

static CONTENT: Lazy<Mutable<String>> = lazy::default();

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::new().center_x())
        .s(Width::fill().max(600))
        .s(Padding::new().top(30).x(20))
        .s(Gap::both(30))
        .item(heading())
        .item(text_input())
        .item(text_display())
}

fn heading() -> impl Element {
    El::with_tag(Tag::H1)
        .s(Align::new().center_x())
        .s(Font::new()
            .color(color!("purple"))
            .size(40)
            .weight(FontWeight::ExtraLight)
            .italic())
        .child("TextArea example")
}

fn text_input() -> impl Element {
    TextArea::new()
        .s(Width::fill())
        .s(Height::exact(200))
        .s(Padding::all(8))
        .s(RoundedCorners::all(8).bottom_right(0))
        .placeholder(Placeholder::new("Write something here..."))
        .on_change(|text| CONTENT.set(text))
        .label_hidden("Write something...")
}

fn text_display() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::default().min(100))
        .s(Padding::all(8))
        .s(RoundedCorners::all(8))
        .s(Background::new().color(color!("silver")))
        .s(Font::new().wrap_anywhere())
        .s(Cursor::new(CursorIcon::Default))
        .item_signal(CONTENT.signal_cloned())
}
