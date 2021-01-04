#![no_std]

use zoon::*;
use shared::{UpMsg, DownMsg, Message};
use std::mem;

blocks!{

    #[s_var]
    fn username() -> String {
        "John".to_owned()
    }

    #[update]
    fn set_username(username: String) {
        username().set(username);
    }

    #[s_var]
    fn messages() -> Vec<VarC<Message>> {
        Vec::new()
    }

    #[s_var]
    fn new_message_text() -> String {
        String::new()
    }

    #[update]
    fn set_new_message_text(text: String) {
        new_message_text().set(text);
    }

    #[s_var]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new(|down_msg, _| {
            if let DownMsg::MessageReceived(message) = down_msg {
                messages().update_mut(|messages| messages.push(new_var_c(message)));
            }
        })
    }

    #[update]
    fn send_message() {
        connection().use_ref(|connection| {
            connection.send_up_msg(UpMsg::SendMessage(Message {
                username: username().inner(),
                text: new_message_text().map_mut(mem::take),
            }), None);
        });
    }

    #[el]
    fn root() -> Column {
        column![
            received_messages(),
            new_message_panel(),
            username_panel(),
        ]
    }

    #[el]
    fn received_messages() -> Column [
        let messages = messages().map(|messages| {
            messages.iter_vars().map(received_message)
        });
        column![
            messages
        ]
    ]

    #[el]
    fn received_message(message: Var<Message>) -> Row {
        let message = message.inner();
        row![
            column![
                el![
                    font::bold(),
                    message.username,
                ],
                message.text
            ]
        ]
    }

    #[el]
    fn new_message_panel() -> Row {
        let new_message_text = new_message_text().inner();
        row![
            text_input![
                do_once(focus),
                text_input::on_change(set_new_message_text),
                input::label_hidden("New message text"),
                placeholder![
                    placeholder::text("Message"),
                ],
                on_key_down(|event| {
                    if let Key::Enter = event.key {
                        send_message()
                    }
                }),
                new_message_text,
            ],
            button![
                button::on_press(send_message), 
                "Send",
            ],
        ]
    }

    #[el]
    fn username_panel() -> Row {
        let input_id = el_var(ElementId::new);
        let username = username().inner();
        row![
            label![
                label::for_input(input_id.inner()),
                "Username:",
            ],
            text_input![
                id(input_id.inner()),
                text_input::on_change(set_username),
                placeholder![
                    placeholder::text("Joe"),
                ],
                username,
            ],
        ]
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
