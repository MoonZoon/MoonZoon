use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Boon Counter example")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
