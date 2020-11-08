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

    #[update]
    fn send_message(text: String) {
        up_queue().update(|queue| {
            queue.push(UpMsg::SendMessage(Message {
                username: username().inner(),
                text,
            }));
        };)
    }

    #[subscription]
    fn message_received() {
        let msg = down_queue().map_mut(MsgQueue::pop);    // @TODO infinite loop?
        if let Some(DownMsg::MessageReceived(message)) = msg {
            messages().update(|messages| messages.push(Model::new(message)));
        }
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
