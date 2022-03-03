use moon::*;

mod custom_config;
use custom_config::CUSTOM_CONFIG;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Custom Config example")
        .append_to_head(
            "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }

            .button {
                background-color: darkgreen;
                padding: 5px;
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
    println!("{:#?}", *CUSTOM_CONFIG);

    start(frontend, up_msg_handler, |_| {}).await
}
