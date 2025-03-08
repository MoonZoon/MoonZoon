use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Boon Lang example")
        .index_by_robots(false)
        .append_to_head(concat!("<style>", include_str!("style.css"), "</style>"))
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
