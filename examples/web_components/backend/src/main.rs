use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Web Components example").append_to_head(
        "
        <style>
            html {
                background-color: black;
            }
        </style>",
    )
    // search component
    .append_to_head(r#"<script type="module" src="https://1.www.s81c.com/common/carbon/web-components/version/v1.24.0/search.min.js"></script>"#)
    // tile component
    .append_to_head(r#"<script type="module" src="https://1.www.s81c.com/common/carbon/web-components/version/v1.24.0/tile.min.js"></script>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
