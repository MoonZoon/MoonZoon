use moon::*;
use shared::{UpMsg, DownMsg, Message};

async fn request_handler(req: Request) {
    if let UpMsg::SendMessage(message) = req.up_msg {
        join_all(connected_client::by_id().iter().map(|(_, client)| {
            client.send_down_msg(message)
        })).await
    }
}

fn main() {
    start!(request_handler);
}
