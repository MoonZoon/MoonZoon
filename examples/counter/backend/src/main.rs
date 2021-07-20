use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Counter example").append_to_head(
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
    let config = Config::from_env_vars();
    println!("Moon config: {:?}", config);
    start(frontend, up_msg_handler, config, |_| {}).await
}
