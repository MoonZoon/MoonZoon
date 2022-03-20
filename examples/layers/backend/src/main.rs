use moon::*;

async fn frontend() -> Frontend {
    // @TODO replace CSS animation with a future Zoon animation API
    // @TODO inspiration: https://github.com/MoonZoon/MoonZoon/blob/main/docs/frontend.md#faq
    Frontend::new().title("Layers example").append_to_head(
        "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }

            .rectangle {
                animation-name: stretch;
                animation-duration: 2.0s;
                animation-timing-function: ease-out;
                animation-direction: alternate;
                animation-iteration-count: infinite;
                animation-play-state: running;
            }
      
            @keyframes stretch {
                100% {
                    transform: scale(1.2);
                }
            }
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
