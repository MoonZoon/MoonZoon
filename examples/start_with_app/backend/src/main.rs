use moon::{
    actix_cors::Cors,
    actix_web::{
        http::StatusCode,
        middleware::{Compat, Condition, ErrorHandlers, Logger},
        web::{self, ServiceConfig},
        App, Responder,
    },
    config::CONFIG,
    *,
};

async fn frontend() -> Frontend {
    Frontend::new().title("Counter example").append_to_head(
        "
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
        </style>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

async fn hello() -> impl Responder {
    "Hello!"
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        let redirect = Redirect::new()
            .http_to_https(CONFIG.https)
            .port(CONFIG.redirect.port, CONFIG.port);

        App::new()
            .wrap(Condition::new(
                CONFIG.redirect.enabled,
                Compat::new(redirect),
            ))
            .wrap(Logger::new("%r %s %D ms %a"))
            .wrap(Cors::default().allowed_origin_fn(move |origin, _| {
                if CONFIG.cors.origins.contains("*") {
                    return true;
                }
                let origin = match origin.to_str() {
                    Ok(origin) => origin,
                    Err(_) => return false,
                };
                CONFIG.cors.origins.contains(origin)
            }))
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error_handler::internal_server_error,
                    )
                    .handler(StatusCode::NOT_FOUND, error_handler::not_found),
            )
    };

    let service_config = |service_config: &mut ServiceConfig| {
        service_config.route("/hello", web::get().to(hello));
    };

    start_with_app(frontend, up_msg_handler, app, service_config).await
}
