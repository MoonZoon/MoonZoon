use zoon::{*, named_color::*};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn content() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//   Commands
// ------ ------

fn set_content(text: String) {
    content().set(text);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::new().center_x())
        .s(Width::fill().max(600))
        .s(Padding::new().top(30).x(20))
        .s(Spacing::new(30))
        .item(heading())
        .item(text_input())
        .item(text_display())
}

fn heading() -> impl Element {
    El::with_tag(Tag::H1)
        .s(Align::new().center_x())
        .s(Font::new().color(PURPLE_7).size(40).weight(FontWeight::ExtraLight).italic())
        .child("TextArea example")
}

fn text_input() -> impl Element {
    TextArea::new()
        .s(Width::fill())
        .s(Height::new(200))
        .s(Padding::all(8))
        .s(RoundedCorners::all(8).bottom_right(0))
        .placeholder(Placeholder::new("Write something here..."))
        .on_change(set_content)
        .label_hidden("Write something...")
}

fn text_display() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::default().min(100))
        .s(Padding::all(8))
        .s(RoundedCorners::all(8))
        .s(Background::new().color(GRAY_2))
        .s(Font::new().wrap_anywhere())
        .s(Cursor::new(CursorIcon::Default))
        .item_signal(content().signal_cloned())
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
