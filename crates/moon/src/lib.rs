use std::{
    borrow::Cow,
    collections::BTreeSet,
    convert::Infallible,
    env,
    error::Error,
    // fs,
    future::Future,
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::{io::AsyncReadExt, runtime::Runtime, signal, sync::mpsc, sync::oneshot, task};
use tokio_stream::wrappers::UnboundedReceiverStream;



use actix_web::rt::System;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Result, post};
use actix_http::http::{header, HeaderMap, HeaderValue, ContentEncoding};
use actix_files::{Files, NamedFile};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::fs;
use std::path::PathBuf;


mod html;
mod sse;

use sse::{Broadcaster, broadcast};

pub struct Frontend {
    title: String,
    append_to_head: String,
    body_content: Cow<'static, str>,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            title: String::new(),
            append_to_head: String::new(),
            body_content: Cow::from(r#"<section id="app"></section>"#),
        }
    }
}

impl Frontend {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn append_to_head(mut self, html: &str) -> Self {
        self.append_to_head.push_str(html);
        self
    }
    pub fn body_content(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        self.body_content = content.into();
        self
    }
}

pub struct UpMsgRequest {}

#[macro_export]
macro_rules! start {
    ($init:expr, $frontend:expr, $up_msg_handler:expr) => {
        $crate::start($init, $frontend, $up_msg_handler);
    };
}

#[derive(Debug)]
struct Config {
    port: u16,
    https: bool,
    redirect_server: RedirectServer,
    compressed_pkg: bool,
}

#[derive(Debug)]
struct RedirectServer {
    port: u16,
    enabled: bool,
}

fn load_config() -> Config {
    // @TODO envy?
    // let config = envy::from_env::<Config>().unwrap();

    // // port = 8443
    // env::set_var("PORT", config.port.to_string());
    // // https = true
    // env::set_var("HTTPS", config.https.to_string());
    //
    // // [redirect_server]
    // // port = 8080
    // env::set_var("REDIRECT_SERVER__PORT", config.redirect_server.port.to_string());
    // // enabled = true
    // env::set_var("REDIRECT_SERVER__ENABLED", config.redirect_server.enabled.to_string());
    //
    // env::set_var("COMPRESSED_PKG", release.to_string());
    Config {
        port: env::var("PORT").map_or(8080, |port| port.parse().unwrap()),
        https: env::var("HTTPS").map_or(false, |https| https.parse().unwrap()),
        redirect_server: RedirectServer {
            port: env::var("REDIRECT_SERVER__PORT").map_or(8081, |port| port.parse().unwrap()),
            enabled: env::var("REDIRECT_SERVER__ENABLED")
                .map_or(false, |enabled| enabled.parse().unwrap()),
        },
        compressed_pkg: env::var("COMPRESSED_PKG")
            .map_or(true, |compressed| compressed.parse().unwrap()),
    }
}

// async fn up_msg_handler_responder<UP>(up_msg_handler: impl Fn(UpMsgRequest) -> UP + Copy + Send + Sync + 'static) -> impl Responder
// where
//     UP: Future<Output = ()> + Send,
// {
//     up_msg_handler(UpMsgRequest {}).await;
//     HttpResponse::Ok()
// }

pub trait UpHandlerReturn: Future<Output = ()> + 'static {}
impl<T> UpHandlerReturn for T where T: Future<Output = ()> + 'static {}

pub trait UpHandler<UPHR: UpHandlerReturn>: Fn(UpMsgRequest) -> UPHR + Clone + Send + 'static {}
impl<T, UPHR: UpHandlerReturn> UpHandler<UPHR> for T where T: Fn(UpMsgRequest) -> UPHR + Clone + Send + 'static {}

async fn up_msg_handler_responder<UPHR, UPH>(up_msg_handler: web::Data<UPH>) -> HttpResponse
where
    UPHR: UpHandlerReturn,
    UPH: UpHandler<UPHR>
{
    up_msg_handler(UpMsgRequest {}).await;
    HttpResponse::Ok().finish()
}

pub fn start<IN, FR, UPHR: UpHandlerReturn, UPH: UpHandler<UPHR>>(
    init: impl FnOnce() -> IN + 'static,
    frontend: impl Fn() -> FR + Copy + Send + Sync + 'static,
    up_msg_handler: UPH,
) -> std::io::Result<()>
where
    IN: Future<Output = ()>,
    FR: Future<Output = Frontend> + Send,
    UPHR: UpHandlerReturn,
    UPH: UpHandler<UPHR>
{
    let config = load_config();
    println!("Moon config: {:#?}", config);

    System::new().block_on(async move {
        let backend_build_id: u128 = fs::read_to_string("backend/private/build_id")
            .await
            .ok()
            .and_then(|uuid| uuid.parse().ok())
            .unwrap_or_default();

        init().await;

        HttpServer::new(move || {
            App::new()
                .service(
                    web::scope("api")
                        .data(up_msg_handler.clone())
                        .route("up_msg_handler", web::post().to(up_msg_handler_responder::<UPHR, UPH>))
                        // .service(reload_resource)
                        // .service(sse_resource);
                )
                // .service(public_service)
                // .service(pkg_resource)
                // .route(frontend_route)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await

        // let broadcaster_data = Broadcaster::create();
        // let broadcaster_data_for_reload = broadcaster_data.clone();
        // let broadcaster_data_for_sse = broadcaster_data.clone();
        
        // let reload_resource = web::resource("reload").route(
        //     web::post().to(move || {
        //         let broadcaster_data = broadcaster_data_for_reload.clone();
        //         async move {
        //             broadcaster_data.lock().unwrap().send("reload", "");
        //             HttpResponse::Ok()
        //         }
        //     })
        // );

        // let sse_resource = web::resource("sse").route(
        //     web::post().to(move || {
        //         let broadcaster_data = broadcaster_data_for_sse.clone();
        //         async move {
        //             let client = broadcaster_data
        //                 .lock()
        //                 .unwrap()
        //                 .new_client("backend_build_id", &backend_build_id.to_string());

        //             HttpResponse::Ok()
        //                 .insert_header(("content-type", "text/event-stream"))
        //                 .no_chunking(100)
        //                 .streaming(client)
        //         }
        //     })
        // );

        


        // let public_service = Files::new("public", "public/");


        // let compressed_pkg = config.compressed_pkg;

        // let pkg_resource = web::resource("pkg/{file:.*}").route(
        //     web::get().to(move |file: web::Path<String>, req: HttpRequest| async move {
        //         let mime = mime_guess::from_path(file.as_str()).first_or_octet_stream();

        //         let encodings = req
        //             .headers()
        //             .get_all(header::ACCEPT_ENCODING)
        //             .collect::<BTreeSet<_>>();

        //         let brotli_header_value = HeaderValue::from_static("br");
        //         let gzip_header_value = HeaderValue::from_static("gzip");

        //         let mut file = file.into_inner();

        //         let named_file = if compressed_pkg || encodings.contains(&brotli_header_value) {
        //             file.push_str(".br");
        //             NamedFile::open(file)?.set_content_encoding(ContentEncoding::Br)
        //         } else if compressed_pkg || encodings.contains(&gzip_header_value) {
        //             file.push_str(".gz");
        //             NamedFile::open(file)?.set_content_encoding(ContentEncoding::Gzip)
        //         } else {
        //             NamedFile::open(file)?
        //         };

        //         Ok::<_, std::io::Error>(named_file.set_content_type(mime))
        //     })
        // );

        // let frontend_route = web::get().to(move || async move {
        //     let frontend = frontend().await;

        //     let frontend_build_id: u128 = fs::read_to_string("frontend/pkg/build_id")
        //         .await
        //         .ok()
        //         .and_then(|uuid| uuid.parse().ok())
        //         .unwrap_or_default();

        //     html::html(
        //         &frontend.title,
        //         backend_build_id,
        //         frontend_build_id,
        //         &frontend.append_to_head,
        //         &frontend.body_content,
        //     )
        // });    
        
        

        // let main_server_routes = up_msg_handler_route
        //     .or(reload)
        //     .or(sse)
        //     .or(pkg_route)
        //     .or(public_route)
        //     .or(frontend_route);

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
    })
    
    // Ok(())
}
