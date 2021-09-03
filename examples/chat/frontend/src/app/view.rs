use zoon::*;

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    El::new()
        .s(Padding::new().y(20))
        .s(Height::screen())
        .child(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::new(300))
        .s(Height::fill())
        .s(Align::new().center_x())
        .s(Spacing::new(20))
        .item(received_messages())
        .item(new_message_panel())
        .item(username_panel())
}

// ------ received_messages ------

fn received_messages() -> impl Element {
    El::new()
        .s(Height::fill())
        .s(Scrollbars::y_and_clip_x())
        .viewport_y_signal(super::received_messages_viewport_y().signal())
        .child(
            Column::new()
                .s(Align::new().bottom())
                .items_signal_vec(super::messages().signal_vec_cloned().map(received_message))
        )
}

fn received_message(message: super::Message) -> impl Element {
    Column::new()
        .s(Padding::all(10))
        .s(Spacing::new(6))
        .item(
            El::new()
                .s(Font::new()
                    .weight(NamedWeight::Bold)
                    .color(NamedColor::Gray10)
                    .size(17))
                .child(message.username),
        )
        .item(
            El::new()
                .s(Font::new().color(NamedColor::Gray8).size(17))
                .child(message.text),
        )
}

// ------ new_message_panel ------

fn new_message_panel() -> impl Element {
    Row::new().item(new_message_input()).item(send_button())
}

fn new_message_input() -> impl Element {
    TextInput::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().left(5))
        .s(Width::fill())
        .s(Font::new().size(17))
        .focus(true)
        .on_change(super::set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Message"))
        .on_key_down(|event| event.if_key(Key::Enter, super::send_message))
        .text_signal(super::new_message_text().signal_cloned())
}

fn send_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().right(5))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2)))
        .s(Font::new().color(NamedColor::Gray10).size(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .on_press(super::send_message)
        .label("Send")
}

// ------ username_panel ------

fn username_panel() -> impl Element {
    let id = "username_input";
    Row::new()
        .s(Spacing::new(15))
        .item(username_input_label(id))
        .item(username_input(id))
}

fn username_input_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(NamedColor::Gray10))
        .for_input(id)
        .label("Username:")
}

fn username_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::fill())
        .s(Padding::new().x(10).y(6))
        .s(RoundedCorners::all(5))
        .id(id)
        .on_change(super::set_username)
        .placeholder(Placeholder::new("Joe"))
        .text_signal(super::username().signal_cloned())
}
