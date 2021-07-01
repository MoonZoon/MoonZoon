use moon::*;
use shared::{DownMsg, UpMsg};

async fn frontend() -> Frontend {
    Frontend::new().title("Chat example").append_to_head(
        "
        <style>
            html {
                background-color: black;
            }
        </style>",
    )
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);

    let UpMsgRequest { up_msg, cor_id, .. } = req;
    let UpMsg::SendMessage(message) = up_msg;

    sessions::broadcast_down_msg(&DownMsg::MessageReceived(message), cor_id).await;
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
