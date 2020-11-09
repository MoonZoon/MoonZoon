use zoon::*;
use shared::{UpMsg, DownMsg, Message};

zoons!{

    #[model]
    fn username() -> String {
        "John".to_owned()
    }

    #[update]
    fn set_username(username: String) {
        username().set(username);
    }

    #[model]
    fn messages() -> Vec<Model<Message>> {
        Vec::new()
    }

    #[model]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new("localhost:9000", |msg| {
            if let DownMsg::MessageReceived(message) = msg {
                messages().update(|messages| messages.push(Model::new(message)));
            }
        })
    }

    #[update]
    fn send_message(text: String) {
        connection().use_ref(|connection| {
            connection.send_msg(UpMsg::SendMessage(Message {
                username: username().inner(),
                text,
            }));
        });
    }

    #[view]
    fn view() -> Column {
        column![
            button![button::on_press(decrement), "-"],
            counter().inner(),
            button![button::on_press(increment), "+"],
        ]
    }

}

fn main() {
    start!(zoons)
}
