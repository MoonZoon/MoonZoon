use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Benchmark example")
        .append_to_head(r#"<link href="/_api/public/css/currentStyle.css" rel="stylesheet"/>"#)
        .body_content(r#"<div id="main"></div>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env_vars();
    println!("Moon config: {:?}", config);
    start(frontend, up_msg_handler, config, |_| {}).await
}
