use moon::*;
use shared::{UpMsg, DownMsg, Message};

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

    let UpMsgRequest { up_msg, cor_id, auth_token } = req;

    if let UpMsg::SendMessage(message) = up_msg {
        // join_all(connected_client::by_id().iter().map(|(_, client)| {
        //     client.send_down_msg(message, req.cor_id)
        // })).await
    }
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_|{}).await
}
