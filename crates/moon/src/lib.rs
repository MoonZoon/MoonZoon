use std::{collections::BTreeSet, future::Future, sync::Mutex};
use std::fs::File;
use std::io::{BufReader, Write, stdout};
use std::net::SocketAddr;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Result};
use actix_web::http::header::ContentType;
use actix_web::middleware::Condition;
use actix_http::http::{header, HeaderValue, ContentEncoding};
use actix_files::{Files, NamedFile};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig as RustlsServerConfig};
use tokio::fs;
pub use trait_set::trait_set;

pub use actix_web;
pub use actix_files;
pub use actix_http;
pub use tokio;
pub use tokio_stream;
pub use mime;
pub use mime_guess;
pub use serde;
pub use futures;
pub use uuid;
pub use actix_web::main as main;

mod config;
mod from_env_vars;
mod frontend;
mod redirect;
mod sse;

use config::Config;
pub use from_env_vars::FromEnvVars;
pub use frontend::Frontend;
pub use redirect::Redirect;
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
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let redirect_enabled = config.redirect_server.enabled;
    let redirect = Redirect::new()
        .http_to_https(config.https)
        .port(config.redirect_server.port, config.port);

    let mut messages = Vec::new();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Condition::new(redirect_enabled, redirect))
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
    });
    
    server = if config.https {
        server.bind_rustls(address, rustls_server_config()?)?
    } else {
        server.bind(address)?
    };

    writeln!(
        &mut messages, 
        "Server is running on {protocol}://{address} [{protocol}://localhost:{port}]",
        address = address,
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    )?;

    server = if config.redirect_server.enabled {
        let address = SocketAddr::from(([0, 0, 0, 0], config.redirect_server.port));
        writeln!(
            &mut messages,
            "Redirect from http://{address} [http://localhost:{port}]",
            address = address,
            port = config.redirect_server.port
        )?;
        server.bind(address)?
    } else {
        server
    };
    let server = server.run();
    
    stdout().write_all(&messages)?;
    
    server.await?;
    println!("Moon shut down");
    Ok(())
}

// fn bind_server(server: HttpServer<>)

fn rustls_server_config() -> std::io::Result<RustlsServerConfig> {
    let mut config = RustlsServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("backend/private/public.pem")?);
    let key_file = &mut BufReader::new(File::open("backend/private/private.pem")?);
    let cert_chain = certs(cert_file).expect("certificate parsing failed");
    let mut keys = pkcs8_private_keys(key_file).expect("private key parsing failed");
    config.set_single_cert(cert_chain, keys.remove(0)).expect("private key is invalid");
    Ok(config)
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
        .insert_header(ContentType(mime::TEXT_EVENT_STREAM))
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


// @TODO fix or remove (with dev-deps)

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod pkg_route {

//         use super::*;
//         use const_format::concatcp;

//         const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
//         const FIXTURES_DIR: &str = concatcp!(MANIFEST_DIR, "/tests/fixtures/");

//         #[tokio::test]
//         async fn uncompressed() {
//             let css_content = include_str!("../tests/fixtures/index.css");
//             let filter = pkg_route(true, FIXTURES_DIR);
//             let res = warp::test::request()
//                 .path("/pkg/index.css")
//                 .reply(&filter)
//                 .await;
//             assert_eq!(res.status(), 200);
//             assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
//             assert_eq!(res.into_body(), css_content);
//         }

//         #[tokio::test]
//         async fn brotli_compressed() {
//             let css_content = include_bytes!("../tests/fixtures/index.css.br");
//             let filter = pkg_route(true, FIXTURES_DIR);
//             let res = warp::test::request()
//                 .header(ACCEPT_ENCODING, "br")
//                 .path("/pkg/index.css")
//                 .reply(&filter)
//                 .await;
//             assert_eq!(res.status(), 200);
//             assert_eq!(res.headers()[CONTENT_ENCODING], "br");
//             assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
//             assert_eq!(res.into_body().as_ref(), css_content);
//         }

//         #[tokio::test]
//         async fn gzip_compressed() {
//             let css_content = include_bytes!("../tests/fixtures/index.css.gz");
//             let filter = pkg_route(true, FIXTURES_DIR);
//             let res = warp::test::request()
//                 .header(ACCEPT_ENCODING, "gzip")
//                 .path("/pkg/index.css")
//                 .reply(&filter)
//                 .await;
//             assert_eq!(res.status(), 200);
//             assert_eq!(res.headers()[CONTENT_ENCODING], "gzip");
//             assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
//             assert_eq!(res.into_body().as_ref(), css_content);
//         }
//     }
// }
