use zoon::{format, *};
use zoon::named_color::{GRAY_0, PURPLE_7};

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
        .s(Height::screen())
        .item(heading())
        .item(text_input())
        .item(text_display())
        .s(Spacing::new(30))
}

fn heading() -> impl Element {
    Row::new()
        .item(Paragraph::new().content("TextArea example")
            .s(Font::new().color(PURPLE_7).size(40).weight(FontWeight::ExtraLight).italic()))
        .s(Align::new().center_x())
}

fn text_input() -> impl Element {
    TextArea::new()
        .placeholder(Placeholder::new("Write something here..."))
        .s(Height::new(300))
        .s(Width::new(600))
        .s(Align::new().center_x())
        .on_change(set_content)
        .label_hidden("Write something...")
}

fn text_display() -> impl Element {
    Column::new()
        .s(Height::new(100))
        .s(Width::new(600))
        .s(Background::new().color(hsluv!(0,0,100)))
        .s(Align::new().center_x())
        .item(Text::with_signal(content().signal_cloned()))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
