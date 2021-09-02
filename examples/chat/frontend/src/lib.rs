use shared::{DownMsg, Message, UpMsg};
use zoon::{eprintln, *};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn username() -> &'static Mutable<String> {
    Mutable::new("John".to_owned())
}

#[static_ref]
fn messages() -> &'static MutableVec<Message> {
    MutableVec::new()
}

#[static_ref]
fn new_message_text() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        messages().lock_mut().push_cloned(message);
        jump_to_bottom();
    })
}

#[static_ref]
fn viewport_y() -> &'static Mutable<i32> {
    Mutable::new(0)
}

// ------ ------
//   Commands
// ------ ------

fn set_username(name: String) {
    username().set(name);
}

fn set_new_message_text(text: String) {
    new_message_text().set(text);
}

fn send_message() {
    Task::start(async {
        connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                username: username().get_cloned(),
                text: new_message_text().take(),
            }))
            .await
            .unwrap_or_else(|error| eprintln!("Failed to send message: {:?}", error))
    });
}

fn jump_to_bottom() {
    viewport_y().set(i32::MAX);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    El::new()
        .s(Padding::new().y(20))
        .s(Scrollbars::y_and_clip_x())
        .s(Height::screen())
        .viewport_y_signal(viewport_y().signal())
        .child(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::new(300))
        .s(Align::new().center_x())
        .s(Spacing::new(20))
        .item(received_messages())
        .item(new_message_panel())
        .item(username_panel())
}

// ------ received_messages ------

fn received_messages() -> impl Element {
    Column::new().items_signal_vec(messages().signal_vec_cloned().map(received_message))
}

fn received_message(message: Message) -> impl Element {
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
        .on_change(set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Message"))
        .on_key_down(|event| event.if_key(Key::Enter, send_message))
        .text_signal(new_message_text().signal_cloned())
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
        .on_press(send_message)
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
        .on_change(set_username)
        .placeholder(Placeholder::new("Joe"))
        .text_signal(username().signal_cloned())
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
    connection();
}
