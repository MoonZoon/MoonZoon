use shared::{DownMsg, Message, UpMsg};
use zoon::{eprintln, *};

pub mod view;

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
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        messages().lock_mut().push_cloned(message);
        jump_to_bottom();
    })
}

#[static_ref]
fn received_messages_viewport_y() -> &'static Mutable<i32> {
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
    received_messages_viewport_y().set(i32::MAX);
}
