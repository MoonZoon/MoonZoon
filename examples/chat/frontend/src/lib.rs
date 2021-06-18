use zoon::*;
use std::mem;
use shared::{UpMsg, DownMsg, Message};

// ------ ------
//    Statics 
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

// #[static_ref]
// fn connection() -> &'static Connection<UpMsg, DownMsg> {
//     Connection::new(|down_msg, _| {
//         if let DownMsg::MessageReceived(message) = down_msg {
//             messages().lock_mut().push_cloned(message);
//         }
//     })
// }

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
    // connection.send_up_msg(UpMsg::SendMessage(Message {
    //     username: username().get_cloned(),
    //     text: new_message_text().map_mut(mem::take),
    // }), None);
}

// ------ ------
//     View 
// ------ ------

fn root() -> impl Element {
    Column::new()
        .item(received_messages())
        .item(new_message_panel())
        .item(username_panel())
}

fn received_messages() -> impl Element {
    Column::new().items_signal_vec(
        messages().signal_vec_cloned().map(received_message)
    )
}

fn received_message(message: Message) -> impl Element {
    Row::new()
        .item(Column::new()
            .item(El::new()
                // .style(Font::new().bold())
                .child(message.username)
            )
            .item(message.text)
        )
}

fn new_message_panel() -> impl Element {
    Row::new()
        .item(new_message_input())
        .item(send_button())
}

fn new_message_input() -> impl Element {
    TextInput::new()
        .focus()
        .on_change(set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Message"))
        // .on_key_down(|event| {
        //     if let Key::Enter = event.key {
        //         send_message()
        //     }
        // })
        .text_signal(new_message_text().signal_cloned())
}

fn send_button() -> impl Element {
    Button::new()
        .on_press(send_message)
        .label("Send")
}

fn username_panel() -> impl Element {
    let id = "username_input";
    Row::new()
        .item(username_input_label(id))
        .item(username_input(id))
}

fn username_input_label(id: &str) -> impl Element {
    Label::new()
        .for_input(id)
        .label("Username:")
}

fn username_input(id: &str) -> impl Element {
    TextInput::new()
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
}
