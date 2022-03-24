use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Javascript example").append_to_head(
        r#"

<script src="//cdn.quilljs.com/1.3.6/quill.js"></script>
<script src="//cdn.quilljs.com/1.3.6/quill.min.js"></script>


<link href="//cdn.quilljs.com/1.3.6/quill.snow.css" rel="stylesheet">
<link href="//cdn.quilljs.com/1.3.6/quill.bubble.css" rel="stylesheet">

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
</style>   
        "#,
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
