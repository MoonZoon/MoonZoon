use actix_files::{Files, NamedFile};
use actix_http::http::{header, ContentEncoding, StatusCode};
use actix_web::http::header::{ContentType, CacheControl, CacheDirective};
use actix_web::middleware::{Condition, Logger, ErrorHandlers, ErrorHandlerResponse};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web::dev::ServiceResponse;
use parking_lot::Mutex;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig as RustlsServerConfig};
use std::fs::File;
use std::io::{self, BufReader};
use std::net::SocketAddr;
use std::{collections::BTreeSet, future::Future};
use tokio::fs;
pub use trait_set::trait_set;

pub use actix_files;
pub use actix_http;
pub use actix_web;
pub use actix_web::main;
pub use futures;
pub use mime;
pub use mime_guess;
pub use parking_lot;
pub use serde;
pub use tokio;
pub use tokio_stream;
pub use uuid;

mod config;
mod from_env_vars;
mod frontend;
mod lazy_message_writer;
mod redirect;
mod sse;

use config::Config;
pub use from_env_vars::FromEnvVars;
pub use frontend::Frontend;
use lazy_message_writer::LazyMessageWriter;
pub use redirect::Redirect;
use sse::{DataSSE, SSE};

pub struct UpMsgRequest {}

#[derive(Copy, Clone)]
struct SharedData {
    backend_build_id: u128,
    compressed_pkg: bool,
    pkg_path: &'static str,
}

// trait aliases
trait_set! {
    pub trait FrontBuilderOutput = Future<Output = Frontend> + 'static;
    pub trait FrontBuilder<FRBO: FrontBuilderOutput> = Fn() -> FRBO + Clone + Send + 'static;

    pub trait UpHandlerOutput = Future<Output = ()> + 'static;
    pub trait UpHandler<UPHO: UpHandlerOutput> = Fn(UpMsgRequest) -> UPHO + Clone + Send + 'static;
}

// ------ ------
//     Start
// ------ ------

pub async fn start<FRB, FRBO, UPH, UPHO>(
    frontend: FRB,
    up_msg_handler: UPH,
    service_config: impl FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static,
) -> io::Result<()>
where
    FRB: FrontBuilder<FRBO>,
    FRBO: FrontBuilderOutput,
    UPH: UpHandler<UPHO>,
    UPHO: UpHandlerOutput,
{
    // ------ Init ------

    let config = Config::from_env_vars();
    println!("Moon config: {:#?}", config);

    env_logger::builder().filter_level(config.backend_log_level).init();

    let shared_data = SharedData {
        backend_build_id: backend_build_id().await,
        compressed_pkg: config.compressed_pkg,
        pkg_path: "frontend/pkg",
    };
    let sse = SSE::start();
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let redirect_enabled = config.redirect.enabled;
    let redirect = Redirect::new()
        .http_to_https(config.https)
        .port(config.redirect.port, config.port);

    let mut lazy_message_writer = LazyMessageWriter::new();

    // ------ App ------

    let mut server = HttpServer::new(move || {
        App::new()
            // https://docs.rs/actix-web/4.0.0-beta.6/actix_web/middleware/struct.Logger.html
            .wrap(Logger::new(r#""%r" %s %b "%{Referer}i" %T"#))
            .wrap(Condition::new(redirect_enabled, redirect))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error_handler)
                    .handler(StatusCode::NOT_FOUND, render_not_found_handler)
            )
            .data(shared_data)
            .data(frontend.clone())
            .data(up_msg_handler.clone())
            .app_data(sse.clone())
            .configure(service_config.clone())
            .service(
                web::scope("_api")
                    .route(
                        "up_msg_handler",
                        web::post().to(up_msg_handler_responder::<UPH, UPHO>),
                    )
                    .route("reload", web::post().to(reload_responder))
                    .service(Files::new("public", "public/"))
                    .route("pkg/{file:.*}", web::get().to(pkg_responder))
                    .route("sse", web::get().to(sse_responder))
                    .route("ping", web::to(|| async { "pong" }))
            )
            .route("*", web::get().to(frontend_responder::<FRB, FRBO>))
    });

    // ------ Bind ------

    server = if config.https {
        server.bind_rustls(address, rustls_server_config()?)?
    } else {
        server.bind(address)?
    };
    lazy_message_writer.server_is_running(&address, &config)?;

    server = if config.redirect.enabled {
        let address = SocketAddr::from(([0, 0, 0, 0], config.redirect.port));
        lazy_message_writer.redirect_from(&address, &config)?;
        server.bind(address)?
    } else {
        server
    };

    // ------ Run ------

    let server = server.run();
    lazy_message_writer.write_all()?;
    server.await?;

    Ok(println!("Stop Moon"))
}

async fn backend_build_id() -> u128 {
    fs::read_to_string("backend/private/build_id")
        .await
        .ok()
        .and_then(|uuid| uuid.parse().ok())
        .unwrap_or_default()
}

fn rustls_server_config() -> io::Result<RustlsServerConfig> {
    let mut config = RustlsServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("backend/private/public.pem")?);
    let key_file = &mut BufReader::new(File::open("backend/private/private.pem")?);
    let cert_chain = certs(cert_file).expect("certificate parsing failed");
    let mut keys = pkcs8_private_keys(key_file).expect("private key parsing failed");
    config
        .set_single_cert(cert_chain, keys.remove(0))
        .expect("private key is invalid");
    Ok(config)
}

// ------ ------
// ErrorHandlers
// ------ ------

fn internal_server_error_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    eprintln!("INTERNAL_SERVER_ERROR: {:#?}", res.request().uri());
    Ok(ErrorHandlerResponse::Response(res))
}

fn render_not_found_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    eprintln!("NOT_FOUND: {:#?}", res.request().uri());
    Ok(ErrorHandlerResponse::Response(res))
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

async fn pkg_responder(
    req: HttpRequest,
    file: web::Path<String>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let mime = mime_guess::from_path(file.as_str()).first_or_octet_stream();

    let encodings = req
        .headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|accept_encoding| accept_encoding.to_str().ok())
        .map(|accept_encoding| accept_encoding.split(", ").collect::<BTreeSet<_>>())
        .unwrap_or_default();

    let mut file = format!("{}/{}", shared_data.pkg_path, file);

    let (named_file, encoding) = match shared_data.compressed_pkg {
        true if encodings.contains(ContentEncoding::Br.as_str()) => {
            file.push_str(".br");
            (NamedFile::open(file)?, Some(ContentEncoding::Br))
        }
        true if encodings.contains(ContentEncoding::Gzip.as_str()) => {
            file.push_str(".gz");
            (NamedFile::open(file)?, Some(ContentEncoding::Gzip))
        }
        _ => (NamedFile::open(file)?, None),
    };

    let mut responder = named_file
        .set_content_type(mime)
        .prefer_utf8(true)
        .use_etag(false)
        .use_last_modified(false)
        .with_header(CacheControl(vec![CacheDirective::MaxAge(31536000)]));

    if let Some(encoding) = encoding {
        responder = responder.with_header(encoding);
    }

    Ok::<_, Error>(responder)
}

// ------ sse_responder ------

async fn sse_responder(
    sse: web::Data<Mutex<SSE>>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let (connection, event_stream) = sse.new_connection();
    let backend_build_id = shared_data.backend_build_id.to_string();

    if connection
        .send("backend_build_id", &backend_build_id)
        .is_err()
    {
        return HttpResponse::InternalServerError()
            .reason("sending backend_build_id failed")
            .finish();
    }

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

// ====== ====== TESTS ====== ======

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{rt as actix_rt, test};
    use const_format::concatcp;
    use futures::StreamExt;

    const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
    const FIXTURES_DIR: &str = concatcp!(MANIFEST_DIR, "/tests/fixtures");

    #[actix_rt::test]
    async fn test_uncompressed() {
        // ------ ARRANGE ------
        let css_content = include_str!("../tests/fixtures/index.css");

        let shared_data = SharedData {
            backend_build_id: u128::default(),
            compressed_pkg: false,
            pkg_path: FIXTURES_DIR,
        };
        let app = test::init_service(
            App::new()
                .data(shared_data)
                .route("_api/pkg/{file:.*}", web::get().to(pkg_responder)),
        )
        .await;
        let req = test::TestRequest::get().uri("/_api/pkg/index.css").to_request();

        // ------ ACT ------
        let mut resp = test::call_service(&app, req).await;

        // ------ ASSERT ------
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap(),
            mime::TEXT_CSS.to_string()
        );
        assert_eq!(
            resp.take_body().into_future().await.0.unwrap().unwrap(),
            css_content
        );
    }

    #[actix_rt::test]
    async fn test_brotli_compressed() {
        // ------ ARRANGE ------
        let css_content = web::Bytes::from_static(include_bytes!("../tests/fixtures/index.css.br"));

        let shared_data = SharedData {
            backend_build_id: u128::default(),
            compressed_pkg: true,
            pkg_path: FIXTURES_DIR,
        };
        let app = test::init_service(
            App::new()
                .data(shared_data)
                .route("_api/pkg/{file:.*}", web::get().to(pkg_responder)),
        )
        .await;
        let req = test::TestRequest::get()
            .insert_header((header::ACCEPT_ENCODING, ContentEncoding::Br.as_str()))
            .uri("/_api/pkg/index.css")
            .to_request();

        // ------ ACT ------
        let mut resp = test::call_service(&app, req).await;

        // ------ ASSERT ------
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap(),
            mime::TEXT_CSS.to_string()
        );
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_ENCODING)
                .unwrap()
                .to_str()
                .unwrap(),
            ContentEncoding::Br.as_str()
        );
        assert_eq!(
            resp.take_body().into_future().await.0.unwrap().unwrap(),
            css_content
        );
    }

    #[actix_rt::test]
    async fn test_gzip_compressed() {
        // ------ ARRANGE ------
        let css_content = web::Bytes::from_static(include_bytes!("../tests/fixtures/index.css.gz"));

        let shared_data = SharedData {
            backend_build_id: u128::default(),
            compressed_pkg: true,
            pkg_path: FIXTURES_DIR,
        };
        let app = test::init_service(
            App::new()
                .data(shared_data)
                .route("_api/pkg/{file:.*}", web::get().to(pkg_responder)),
        )
        .await;
        let req = test::TestRequest::get()
            .insert_header((header::ACCEPT_ENCODING, ContentEncoding::Gzip.as_str()))
            .uri("/_api/pkg/index.css")
            .to_request();

        // ------ ACT ------
        let mut resp = test::call_service(&app, req).await;

        // ------ ASSERT ------
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap(),
            mime::TEXT_CSS.to_string()
        );
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_ENCODING)
                .unwrap()
                .to_str()
                .unwrap(),
            ContentEncoding::Gzip.as_str()
        );
        assert_eq!(
            resp.take_body().into_future().await.0.unwrap().unwrap(),
            css_content
        );
    }
}
