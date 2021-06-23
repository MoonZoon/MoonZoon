use moon::*;
use shared::{UpMsg, DownMsg};

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Chat example")
        .append_to_head("<style>html {
            background-color: black;
            color: lightgray;
        }
        
        #app * {
            padding: 5px;
        }
            
        .button {
            cursor: pointer;
            background-color: darkgreen;
            user-select: none;
        }
        
        .button:hover {
            background-color: green;
        }
        
        .column {
            display: flex;
            flex-direction: column;
        }
        
        .row {
            display: flex;
        }
        </style>")
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:#?}", req);

    let UpMsgRequest { up_msg, session_id, cor_id, .. } = req;
    let UpMsg::SendMessage(message) = up_msg;

    let down_msg = DownMsg::MessageReceived(message);

    sessions::broadcast_down_msg(&down_msg, cor_id).await;
    sessions::by_session_id()
        .get(session_id)
        .unwrap()
        .send_down_msg(&down_msg, cor_id)
        .await;
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_|{}).await
}
