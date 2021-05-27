use std::{collections::BTreeSet, future::Future, sync::Mutex};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Result};
use actix_http::http::{header, HeaderValue, ContentEncoding};
use actix_web::http::header::ContentType;
use actix_files::{Files, NamedFile};
use tokio::fs;
pub use trait_set::trait_set;

pub use actix_web;
pub use actix_files;
pub use actix_http;
pub use tokio;
pub use tokio_stream;
pub use mime_guess;
pub use serde;
pub use futures;
pub use uuid;
pub use actix_web::main as main;

mod config;
mod from_env_vars;
mod frontend;
mod sse;

use config::Config;
pub use from_env_vars::FromEnvVars;
pub use frontend::Frontend;
use sse::{SSE, DataSSE};

pub struct UpMsgRequest {}

#[derive(Copy, Clone)]
struct SharedData {
    compressed_pkg: bool,
    backend_build_id: u128,
}

// trait aliases
trait_set!{
    pub trait FrontBuilderOutput = Future<Output = Frontend> + 'static;
    pub trait FrontBuilder<FRBO: FrontBuilderOutput> = Fn() -> FRBO + Clone + Send + 'static;

    pub trait UpHandlerOutput = Future<Output = ()> + 'static;
    pub trait UpHandler<UPHO: UpHandlerOutput> = Fn(UpMsgRequest) -> UPHO + Clone + Send + 'static;
}

pub async fn start<FRB, FRBO, UPH, UPHO>(
    frontend: FRB,
    up_msg_handler: UPH,
    service_config: impl FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static,
) -> std::io::Result<()>
where
    FRB: FrontBuilder<FRBO>,
    FRBO: FrontBuilderOutput,
    UPH: UpHandler<UPHO>,
    UPHO: UpHandlerOutput,
{
    let config = Config::from_env_vars();
    println!("Moon config: {:#?}", config);

    let shared_data = SharedData {
        backend_build_id: backend_build_id().await,
        compressed_pkg: config.compressed_pkg,
    };
    let sse = SSE::start();

    HttpServer::new(move || {
        App::new()
            .data(shared_data)
            .data(frontend.clone())
            .data(up_msg_handler.clone())
            .app_data(sse.clone())
            .configure(service_config.clone())
            .service(
                web::scope("api")
                    .route("up_msg_handler", web::post().to(up_msg_handler_responder::<UPH, UPHO>))
                    .route("reload", web::post().to(reload_responder))
            )
            .service(Files::new("public", "public/"))
            .route("pkg/{file:.*}", web::get().to(pkg_responder))
            .route("sse", web::get().to(sse_responder))
            .route("*", web::get().to(frontend_responder::<FRB, FRBO>))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await        


    // let (shutdown_sender_for_redirect_server, redirect_server_handle) = {
    //     let config_port = config.port;
    //     let config_https = config.https;

    //     if config.redirect_server.enabled {
    //         let redirect_server_routes = warp::path::full().and(warp::host::optional()).map(
    //             move |path: FullPath, authority: Option<Authority>| {
    //                 let authority = authority.unwrap();
    //                 let authority = format!("{}:{}", authority.host(), config_port);
    //                 let authority = authority.parse::<Authority>().unwrap();

    //                 let uri = Uri::builder()
    //                     .scheme(if config_https { "https" } else { "http" })
    //                     .authority(authority)
    //                     .path_and_query(path.as_str())
    //                     .build()
    //                     .unwrap();
    //                 warp::redirect::temporary(uri)
    //             },
    //         );

    //         let (shutdown_sender_for_redirect_server, shutdown_receiver_for_redirect_server) =
    //             oneshot::channel();
    //         let (_, redirect_server) = warp::serve(redirect_server_routes)
    //             .bind_with_graceful_shutdown(
    //                 ([0, 0, 0, 0], config.redirect_server.port),
    //                 async {
    //                     shutdown_receiver_for_redirect_server.await.ok();
    //                 },
    //             );
    //         let redirect_server_handle = task::spawn(redirect_server);

    //         (
    //             Some(shutdown_sender_for_redirect_server),
    //             Some(redirect_server_handle),
    //         )
    //     } else {
    //         (None, None)
    //     }
    // };

    // let (shutdown_sender_for_main_server, shutdown_receiver_for_main_server) =
    //     oneshot::channel();
    // let main_server_handle = {
    //     let server = warp::serve(main_server_routes);
    //     if config.https {
    //         let main_server = server
    //             .tls()
    //             .cert_path("backend/private/public.pem")
    //             .key_path("backend/private/private.pem")
    //             .bind_with_graceful_shutdown(([0, 0, 0, 0], config.port), async {
    //                 shutdown_receiver_for_main_server.await.ok();
    //             })
    //             .1;
    //         task::spawn(main_server)
    //     } else {
    //         let main_server = server
    //             .bind_with_graceful_shutdown(([0, 0, 0, 0], config.port), async {
    //                 shutdown_receiver_for_main_server.await.ok();
    //             })
    //             .1;
    //         task::spawn(main_server)
    //     }
    // };

    // if config.redirect_server.enabled {
    //     println!(
    //         "Redirect server is running on 0.0.0.0:{port} [http://127.0.0.1:{port}]",
    //         port = config.redirect_server.port
    //     );
    // }
    // println!(
    //     "Main server is running on 0.0.0.0:{port} [{protocol}://127.0.0.1:{port}]",
    //     protocol = if config.https { "https" } else { "http" },
    //     port = config.port
    // );

    // signal::ctrl_c().await.unwrap();
    // if let Some(shutdown_sender_for_redirect_server) = shutdown_sender_for_redirect_server {
    //     shutdown_sender_for_redirect_server.send(()).unwrap();
    // }
    // shutdown_sender_for_main_server.send(()).unwrap();
    // // time::sleep(time::Duration::from_secs(1)).await;
    // if let Some(redirect_server_handle) = &redirect_server_handle {
    //     redirect_server_handle.abort();
    // }
    // main_server_handle.abort();

    // let mut handles = vec![main_server_handle];
    // if let Some(redirect_server_handle) = redirect_server_handle {
    //     handles.push(redirect_server_handle);
    // }
    // futures::future::join_all(handles).await;

    // println!("Moon shut down");
    
    // Ok(())
}

async fn backend_build_id() -> u128 {
    fs::read_to_string("backend/private/build_id")
        .await
        .ok()
        .and_then(|uuid| uuid.parse().ok())
        .unwrap_or_default()
}

// ------ ------
//  Responders
// ------ ------

// ------ up_msg_handler_responder ------

async fn up_msg_handler_responder<UPH, UPHO>(up_msg_handler: web::Data<UPH>) -> impl Responder
where
    UPH: UpHandler<UPHO>,
    UPHO: UpHandlerOutput,
{
    up_msg_handler(UpMsgRequest {}).await;
    HttpResponse::Ok()
}

// ------ reload_responder ------

async fn reload_responder(sse: web::Data<Mutex<SSE>>) -> impl Responder {
    let _ = sse.broadcast("reload", "");
    HttpResponse::Ok()
}   

// ------ pkg_responder ------

async fn pkg_responder(req: HttpRequest, file: web::Path<String>, shared_data: web::Data<SharedData>) -> Result<NamedFile> {
    let mime = mime_guess::from_path(file.as_str()).first_or_octet_stream();

    let encodings = req
        .headers()
        .get_all(header::ACCEPT_ENCODING)
        .collect::<BTreeSet<_>>();

    let brotli_header_value = HeaderValue::from_static("br");
    let gzip_header_value = HeaderValue::from_static("gzip");

    let mut file = format!("frontend/pkg/{}", file);

    let named_file = match shared_data.compressed_pkg {
        true if encodings.contains(&brotli_header_value) => {
            file.push_str(".br");
            NamedFile::open(file)?.set_content_encoding(ContentEncoding::Br)
        }
        true if encodings.contains(&gzip_header_value) => {
            file.push_str(".gz");
            NamedFile::open(file)?.set_content_encoding(ContentEncoding::Gzip)
        }
        _ => NamedFile::open(file)?
    };

    Ok(named_file.set_content_type(mime))
}   

// ------ sse_responder ------

async fn sse_responder(sse: web::Data<Mutex<SSE>>, shared_data: web::Data<SharedData>) -> impl Responder {
    let (connection, event_stream) = sse.new_connection();

    connection.send("backend_build_id", &shared_data.backend_build_id.to_string()).unwrap();

    HttpResponse::Ok()
        .insert_header(("content-type", "text/event-stream"))
        .streaming(event_stream)
}   

// ------ frontend_responder ------

async fn frontend_responder<FRB, FRBO>(frontend: web::Data<FRB>) -> impl Responder
where
    FRB: FrontBuilder<FRBO>,
    FRBO: FrontBuilderOutput,
{
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(frontend().await.render().await)
}   
