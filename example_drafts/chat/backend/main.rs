use moon::*;
use shared::{UpMsg, DownMsg, Message};

async fn up_msg_handler(msg: UpMsg) -> Option<DownMsg> {
    if let UpMsg::SendMessage(message) = msg {
        broadcast_down_message(message).await;
    }
    None
}

fn main() {
    start!(up_msg_handler);
}
