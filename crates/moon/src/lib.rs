use actix_files::{Files, NamedFile};
use actix_http::http::{header, ContentEncoding, StatusCode, HeaderMap};
use actix_web::dev::ServiceResponse;
use actix_web::http::header::{CacheControl, CacheDirective, ContentType, ETag, EntityTag};
use actix_web::middleware::{Condition, ErrorHandlerResponse, ErrorHandlers, Logger, Compat};
use actix_web::{web, App, error::{self, Error}, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig as RustlsServerConfig};
use std::fs::File;
use std::io::{self, BufReader};
use std::net::SocketAddr;
use std::{collections::BTreeSet, future::Future};
use std::ops::Deref;
use tokio::fs;
use moonlight::{serde_lite::Deserialize, serde_json, CorId, AuthToken};
use futures::StreamExt;

pub use trait_set::trait_set;
pub use actix_files;
pub use actix_http;
pub use actix_web;
pub use actix_web_codegen::main;
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
mod up_msg_request;

use config::Config;
pub use from_env_vars::FromEnvVars;
pub use frontend::Frontend;
use lazy_message_writer::LazyMessageWriter;
pub use redirect::Redirect;
use sse::{ShareableSSE, ShareableSSEMethods, SSE};
pub use up_msg_request::UpMsgRequest;

const MAX_UP_MSG_BYTES: usize = 2 * 1_048_576;

#[derive(Copy, Clone)]
struct SharedData {
    backend_build_id: u128,
    frontend_build_id: u128,
    cache_busting: bool,
    compressed_pkg: bool,
    pkg_path: &'static str,
}

#[derive(Clone)]
struct ReloadSSE(ShareableSSE);

impl Deref for ReloadSSE {
    type Target = ShareableSSE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
struct MessageSSE(ShareableSSE);

impl Deref for MessageSSE {
    type Target = ShareableSSE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// trait aliases
trait_set! {
    pub trait FrontBuilderOutput = Future<Output = Frontend> + 'static;
    pub trait FrontBuilder<FRBO: FrontBuilderOutput> = Fn() -> FRBO + Clone + Send + 'static;

    pub trait UpHandlerOutput = Future<Output = ()> + 'static;
    pub trait UpHandler<UPHO: UpHandlerOutput, UMsg> = Fn(UpMsgRequest<UMsg>) -> UPHO + Clone + Send + 'static;
}

// ------ ------
//     Start
// ------ ------

pub async fn start<'de, FRB, FRBO, UPH, UPHO, UMsg>(
    frontend: FRB,
    up_msg_handler: UPH,
    service_config: impl FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static,
) -> io::Result<()>
where
    FRB: FrontBuilder<FRBO>,
    FRBO: FrontBuilderOutput,
    UPH: UpHandler<UPHO, UMsg>,
    UPHO: UpHandlerOutput,
    UMsg: 'static + Deserialize,
{
    // ------ Init ------

    let config = Config::from_env_vars();
    println!("Moon config: {:#?}", config);

    env_logger::builder()
        .filter_level(config.backend_log_level)
        .init();

    let shared_data = SharedData {
        backend_build_id: backend_build_id().await,
        frontend_build_id: Frontend::build_id().await,
        cache_busting: config.cache_busting,
        compressed_pkg: config.compressed_pkg,
        pkg_path: "frontend/pkg",
    };
    let reload_sse = ReloadSSE(SSE::start());
    let message_sse = MessageSSE(SSE::start());
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let redirect_enabled = config.redirect.enabled;
    let redirect = Redirect::new()
        .http_to_https(config.https)
        .port(config.redirect.port, config.port);

    let mut lazy_message_writer = LazyMessageWriter::new();

    // ------ App ------

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Condition::new(redirect_enabled, Compat::new(redirect)))
            // https://docs.rs/actix-web/4.0.0-beta.6/actix_web/middleware/struct.Logger.html
            .wrap(Logger::new(r#""%r" %s %b "%{Referer}i" %T"#))
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        internal_server_error_handler,
                    )
                    .handler(StatusCode::NOT_FOUND, render_not_found_handler),
            )
            .data(shared_data)
            .data(frontend.clone())
            .data(up_msg_handler.clone())
            .data(reload_sse.clone())
            .data(message_sse.clone())
            .configure(service_config.clone())
            .service(Files::new("_api/public", "public"))
            .service(
                web::scope("_api")
                    .route(
                        "up_msg_handler",
                        web::post().to(up_msg_handler_responder::<UPH, UPHO, UMsg>),
                    )
                    .route("reload", web::post().to(reload_responder))
                    .route("pkg/{file:.*}", web::get().to(pkg_responder))
                    .route("message_sse", web::get().to(message_sse_responder))
                    .route("reload_sse", web::get().to(reload_sse_responder))
                    .route("ping", web::to(|| async { "pong" })),
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

async fn up_msg_handler_responder<UPH, UPHO, UMsg>(
    req: HttpRequest, 
    payload: web::Payload, 
    up_msg_handler: web::Data<UPH>
) -> Result<HttpResponse, Error>
where
    UPH: UpHandler<UPHO, UMsg>,
    UPHO: UpHandlerOutput,
    UMsg: Deserialize,
{
    let headers = req.headers();

    let up_msg_request = UpMsgRequest {
        up_msg: parse_up_msg(payload).await?,
        cor_id: parse_cor_id(headers)?,
        auth_token: parse_auth_token(headers)?,
    };
    up_msg_handler(up_msg_request).await;
    Ok(HttpResponse::Ok().finish())
}

async fn parse_up_msg<UMsg: Deserialize>(mut payload: web::Payload) -> Result<UMsg, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_UP_MSG_BYTES {
            Err(error::JsonPayloadError::Overflow { limit: MAX_UP_MSG_BYTES })?
        }
        body.extend_from_slice(&chunk);
    }
    UMsg::deserialize(
        &serde_json::from_slice(&body).map_err(error::JsonPayloadError::Deserialize)?
    ).map_err(error::ErrorBadRequest)
}

fn parse_cor_id(headers: &HeaderMap) -> Result<CorId, Error> {
    headers
        .get("X-Correlation-ID")
        .ok_or_else(|| error::ErrorBadRequest("header 'X-Correlation-ID' is missing"))?
        .to_str()
        .map_err(error::ErrorBadRequest)?
        .parse()
        .map_err(error::ErrorBadRequest)
}

fn parse_auth_token(headers: &HeaderMap) -> Result<Option<AuthToken>, Error> {
    if let Some(auth_token) = headers.get("X-Auth-Token") {
        let auth_token = auth_token
            .to_str()
            .map_err(error::ErrorBadRequest)
            .map(AuthToken::new)?;
        return Ok(Some(auth_token))
    }
    Ok(None)
}

// ------ reload_responder ------

async fn reload_responder(sse: web::Data<ReloadSSE>) -> impl Responder {
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
    let (named_file, encoding) = named_file_and_encoding(&req, &file, &shared_data)?;

    let named_file = named_file
        .set_content_type(mime)
        .prefer_utf8(true)
        .use_etag(false)
        .use_last_modified(false)
        .disable_content_disposition();

    let mut responder = if shared_data.cache_busting {
        named_file.with_header(CacheControl(vec![CacheDirective::MaxAge(31536000)]))
    } else {
        named_file.with_header(ETag(EntityTag::new(
            false,
            shared_data.frontend_build_id.to_string(),
        )))
    };

    if let Some(encoding) = encoding {
        responder = responder.with_header(encoding);
    }
    Ok::<_, Error>(responder)
}

fn named_file_and_encoding(
    req: &HttpRequest,
    file: &web::Path<String>,
    shared_data: &web::Data<SharedData>,
) -> Result<(NamedFile, Option<ContentEncoding>), Error> {
    let mut file = format!("{}/{}", shared_data.pkg_path, file);
    if !shared_data.compressed_pkg {
        return Ok((NamedFile::open(file)?, None));
    }
    let accept_encodings = req
        .headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|accept_encoding| accept_encoding.to_str().ok())
        .map(|accept_encoding| accept_encoding.split(", ").collect::<BTreeSet<_>>())
        .unwrap_or_default();

    if accept_encodings.contains(ContentEncoding::Br.as_str()) {
        file.push_str(".br");
        return Ok((NamedFile::open(file)?, Some(ContentEncoding::Br)));
    }
    if accept_encodings.contains(ContentEncoding::Gzip.as_str()) {
        file.push_str(".gz");
        return Ok((NamedFile::open(file)?, Some(ContentEncoding::Gzip)));
    }
    Ok((NamedFile::open(file)?, None))
}

// ------ reload_sse_responder ------

async fn reload_sse_responder(
    sse: web::Data<ReloadSSE>,
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
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .streaming(event_stream)
}

// ------ message_sse_responder ------

async fn message_sse_responder(
    sse: web::Data<MessageSSE>,
) -> impl Responder {
    let (_, event_stream) = sse.new_connection();

    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_EVENT_STREAM))
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .streaming(event_stream)
}

// ------ frontend_responder ------

async fn frontend_responder<FRB, FRBO>(
    frontend: web::Data<FRB>,
    shared_data: web::Data<SharedData>,
) -> impl Responder
where
    FRB: FrontBuilder<FRBO>,
    FRBO: FrontBuilderOutput,
{
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(frontend().await.render(shared_data.cache_busting).await)
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
            frontend_build_id: u128::default(),
            backend_build_id: u128::default(),
            cache_busting: bool::default(),
            compressed_pkg: false,
            pkg_path: FIXTURES_DIR,
        };
        let app = test::init_service(
            App::new()
                .data(shared_data)
                .route("_api/pkg/{file:.*}", web::get().to(pkg_responder)),
        )
        .await;
        let req = test::TestRequest::get()
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
            mime::TEXT_CSS_UTF_8.to_string()
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
            frontend_build_id: u128::default(),
            backend_build_id: u128::default(),
            cache_busting: bool::default(),
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
            mime::TEXT_CSS_UTF_8.to_string()
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
            frontend_build_id: u128::default(),
            backend_build_id: u128::default(),
            cache_busting: bool::default(),
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
            mime::TEXT_CSS_UTF_8.to_string()
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
