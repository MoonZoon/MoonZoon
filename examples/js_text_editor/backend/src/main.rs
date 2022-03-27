use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Js text editor example")
        .append_to_head(
            r#"
                <style>
                    html {
                        background: lightgray;
                    }
                </style>   
            "#,
        )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
