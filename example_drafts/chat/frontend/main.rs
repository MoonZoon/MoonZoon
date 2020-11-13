use zoon::*;
use shared::{UpMsg, DownMsg, Message};
use std::mem;

blocks!{

    #[var]
    fn username() -> String {
        "John".to_owned()
    }

    #[update]
    fn set_username(username: String) {
        username().set(username);
    }

    #[var]
    fn messages() -> Vec<Var<Message>> {
        Vec::new()
    }

    #[var]
    fn new_message_text() -> String {
        String::new()
    }

    #[update]
    fn set_new_message_text(text: String) {
        new_message_text().set(text);
    }

    #[var]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new("localhost:9000", |msg, _| {
            if let DownMsg::MessageReceived(message) = msg {
                messages().update_mut(|messages| messages.push(Var::new(message)));
            }
        })
    }

    #[update]
    fn send_message() {
        connection().use_ref(|connection| {
            connection.send_msg(UpMsg::SendMessage(Message {
                username: username().inner(),
                text: new_message_text().map_mut(mem::take),
            }));
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
        column![
            messages().map(|messages| messages.iter().map(received_message)),
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

fn main() {
    start!()
}
