use moon::{
    actix_web::{web, HttpResponse, Responder},
    *,
};

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Custom HTTP client example")
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

async fn up_msg_handler(_: UpMsgRequest<()>) {}

// @TODO Actix macros are broken because of
// https://github.com/actix/actix-web/commit/81ef12a0fd0b982a43e120f2c0afc1b65772a189
// #[get("/_api/moonzoon_stars")]

async fn moonzoon_stars() -> impl Responder {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "serde")]
    struct GithubResponse {
        stargazers_count: u32,
    }

    async fn stars_request() -> reqwest::Result<u32> {
        reqwest::Client::builder()
            // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
            .user_agent("MoonZoon")
            .build()?
            .get("https://api.github.com/repos/MoonZoon/MoonZoon")
            .send()
            .await?
            .json::<GithubResponse>()
            .await
            .map(|github_response| github_response.stargazers_count)
    }

    stars_request().await.map_or_else(
        |error| {
            eprintln!("Error: {:#?}", error);
            HttpResponse::InternalServerError().finish()
        },
        |stars| HttpResponse::Ok().body(stars.to_string()),
    )
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |cfg| {
        cfg.route("/_api/moonzoon_stars", web::get().to(moonzoon_stars));
    })
    .await
}
