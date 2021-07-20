use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("SVG example")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env_vars();
    println!("Moon config: {:?}", config);
    start(frontend, up_msg_handler, config, |_| {}).await
}
