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
        .append_to_head(r#"<link href="//cdn.quilljs.com/1.3.6/quill.snow.css" rel="stylesheet">"#)
        .append_to_head(r#"<script src="//cdn.quilljs.com/1.3.6/quill.min.js"></script>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
