use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Web Workers example")
        // https://github.com/sindresorhus/github-markdown-css
        .append_to_head(concat!(
            "<style>",
            include_str!("styles/github-markdown-light.css"),
            "</style>"
        ))
        .append_to_head(concat!(
            "<style>",
            include_str!("styles/style.css"),
            "</style>"
        ))
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
