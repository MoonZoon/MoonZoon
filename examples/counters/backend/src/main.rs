use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Counters example").append_to_head(
        "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }
            
            #app * {
                padding: 5px;
            }
                
            .button {
                background-color: darkgreen;
            }
            
            .button:hover {
                background-color: green;
            }
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
