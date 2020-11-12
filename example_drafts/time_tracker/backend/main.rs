use moon::*;
use shared::{UpMsg, DownMsg, Message};

moons!{
    #[var]
    fn connector() -> Connector<UpMsg, DownMsg> {
        Connector::new("9000", |msg| {
            if let UpMsg::SendMessage(message) = msg {
                broadcast_message(message);
            }
        })
    }

    #[update]
    fn broadcast_message(message: Message) {
        connector().use_ref(move |connector| {
            connector.broadcast(DownMsg::MessageReceived(message))
        })
    }
}

fn main() {
    start!()
}
