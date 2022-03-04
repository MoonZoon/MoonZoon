use moon::*;
use shared::{DownMsg, UpMsg};

mod custom_config;
use custom_config::CUSTOM_CONFIG;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Custom Config example")
        .append_to_head(
            "
        <style>
            html {
                background-color: black;
                color: lightgray;
            }
        </style>",
        )
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);
    let (session_id, cor_id) = (req.session_id, req.cor_id);

    let favorite_languages = CUSTOM_CONFIG.favorite_languages.join(",");

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        return session.send_down_msg(&DownMsg::FavoriteLanguages(favorite_languages), cor_id).await;
    }
    eprintln!("cannot find the session with id `{}`", session_id);
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    println!("{:#?}", *CUSTOM_CONFIG);

    start(frontend, up_msg_handler, |_| {}).await
}
