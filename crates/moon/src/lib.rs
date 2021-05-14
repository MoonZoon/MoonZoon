use std::{
    borrow::Cow,
    collections::BTreeSet,
    convert::Infallible,
    env,
    error::Error,
    fs,
    future::Future,
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::{io::AsyncReadExt, runtime::Runtime, signal, sync::mpsc, sync::oneshot, task};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    filters::BoxedFilter,
    host::Authority,
    http::{
        self,
        header::{
            HeaderMap, HeaderValue, ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_LENGTH, CONTENT_TYPE,
        },
        Uri,
    },
    path,
    path::FullPath,
    sse::Event,
    Filter, Reply,
};

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

type SseSenders = Vec<mpsc::UnboundedSender<Result<Event, Infallible>>>;

pub fn start<IN, FR, UP>(
    init: impl FnOnce() -> IN,
    frontend: impl Fn() -> FR + Copy + Send + Sync + 'static,
    up_msg_handler: impl Fn(UpMsgRequest) -> UP + Copy + Send + Sync + 'static,
) -> Result<(), Box<dyn Error>>
where
    IN: Future<Output = ()>,
    FR: Future<Output = Frontend> + Send,
    UP: Future<Output = ()> + Send,
{
    let config = load_config();
    println!("Moon config: {:#?}", config);

    let rt = Runtime::new()?;
    rt.block_on(async move {
        let sse_senders = SseSenders::new();
        let sse_senders = Arc::new(Mutex::new(sse_senders));
        let sse_senders = warp::any().map(move || sse_senders.clone());

        let backend_build_id: u128 = fs::read_to_string("backend/private/build_id")
            .ok()
            .and_then(|uuid| uuid.parse().ok())
            .unwrap_or_default();

        init().await;

        let api = warp::post().and(warp::path("api"));

        let up_msg_handler_route =
            api.and(warp::path("up_msg_handler"))
                .and_then(move || async move {
                    up_msg_handler(UpMsgRequest {}).await;
                    Ok::<_, warp::Rejection>(http::StatusCode::OK)
                });

        let reload = api.and(warp::path("reload")).and(sse_senders.clone()).map(
            |sse_senders: Arc<Mutex<SseSenders>>| {
                sse_senders.lock().unwrap().retain(|sse_sender| {
                    sse_sender
                        .send(Ok(Event::default().event("reload").data("")))
                        .is_ok()
                });
                http::StatusCode::OK
            },
        );

        let sse =
            warp::path!("sse")
                .and(sse_senders)
                .map(move |sse_senders: Arc<Mutex<SseSenders>>| {
                    let (sse_sender, sse_receiver) = mpsc::unbounded_channel();
                    let sse_stream =
                        UnboundedReceiverStream::<Result<Event, Infallible>>::new(sse_receiver);

                    let backend_build_id = backend_build_id.to_string();
                    sse_sender
                        .send(Ok(Event::default()
                            .event("backend_build_id")
                            .data(backend_build_id)))
                        .unwrap();

                    sse_senders.lock().unwrap().push(sse_sender);
                    warp::sse::reply(warp::sse::keep_alive().stream(sse_stream))
                });

        let pkg_route = pkg_route(config.compressed_pkg, "frontend/pkg/");
        let public_route = warp::path("public").and(warp::fs::dir("public/"));

        let frontend_route = warp::get().and_then(move || async move {
            let frontend = frontend().await;

            let frontend_build_id: u128 = fs::read_to_string("frontend/pkg/build_id")
                .ok()
                .and_then(|uuid| uuid.parse().ok())
                .unwrap_or_default();

            Ok::<_, warp::Rejection>(warp::reply::html(html(
                &frontend.title,
                backend_build_id,
                frontend_build_id,
                &frontend.append_to_head,
                &frontend.body_content,
            )))
        });

        let main_server_routes = up_msg_handler_route
            .or(reload)
            .or(sse)
            .or(pkg_route)
            .or(public_route)
            .or(frontend_route);

        let (shutdown_sender_for_redirect_server, redirect_server_handle) = {
            let config_port = config.port;
            let config_https = config.https;

            if config.redirect_server.enabled {
                let redirect_server_routes = warp::path::full().and(warp::host::optional()).map(
                    move |path: FullPath, authority: Option<Authority>| {
                        let authority = authority.unwrap();
                        let authority = format!("{}:{}", authority.host(), config_port);
                        let authority = authority.parse::<Authority>().unwrap();

                        let uri = Uri::builder()
                            .scheme(if config_https { "https" } else { "http" })
                            .authority(authority)
                            .path_and_query(path.as_str())
                            .build()
                            .unwrap();
                        warp::redirect::temporary(uri)
                    },
                );

                let (shutdown_sender_for_redirect_server, shutdown_receiver_for_redirect_server) =
                    oneshot::channel();
                let (_, redirect_server) = warp::serve(redirect_server_routes)
                    .bind_with_graceful_shutdown(
                        ([0, 0, 0, 0], config.redirect_server.port),
                        async {
                            shutdown_receiver_for_redirect_server.await.ok();
                        },
                    );
                let redirect_server_handle = task::spawn(redirect_server);

                (
                    Some(shutdown_sender_for_redirect_server),
                    Some(redirect_server_handle),
                )
            } else {
                (None, None)
            }
        };

        let (shutdown_sender_for_main_server, shutdown_receiver_for_main_server) =
            oneshot::channel();
        let main_server_handle = {
            let server = warp::serve(main_server_routes);
            if config.https {
                let main_server = server
                    .tls()
                    .cert_path("backend/private/public.pem")
                    .key_path("backend/private/private.pem")
                    .bind_with_graceful_shutdown(([0, 0, 0, 0], config.port), async {
                        shutdown_receiver_for_main_server.await.ok();
                    })
                    .1;
                task::spawn(main_server)
            } else {
                let main_server = server
                    .bind_with_graceful_shutdown(([0, 0, 0, 0], config.port), async {
                        shutdown_receiver_for_main_server.await.ok();
                    })
                    .1;
                task::spawn(main_server)
            }
        };

        if config.redirect_server.enabled {
            println!(
                "Redirect server is running on 0.0.0.0:{port} [http://127.0.0.1:{port}]",
                port = config.redirect_server.port
            );
        }
        println!(
            "Main server is running on 0.0.0.0:{port} [{protocol}://127.0.0.1:{port}]",
            protocol = if config.https { "https" } else { "http" },
            port = config.port
        );

        signal::ctrl_c().await.unwrap();
        if let Some(shutdown_sender_for_redirect_server) = shutdown_sender_for_redirect_server {
            shutdown_sender_for_redirect_server.send(()).unwrap();
        }
        shutdown_sender_for_main_server.send(()).unwrap();
        // time::sleep(time::Duration::from_secs(1)).await;
        if let Some(redirect_server_handle) = &redirect_server_handle {
            redirect_server_handle.abort();
        }
        main_server_handle.abort();

        let mut handles = vec![main_server_handle];
        if let Some(redirect_server_handle) = redirect_server_handle {
            handles.push(redirect_server_handle);
        }
        futures::future::join_all(handles).await;

        println!("Moon shut down");
    });
    Ok(())
}

const BROTLI_ID: &str = "br";
const BROTLI_POSTFIX: &str = ".br";

const GZIP_ID: &str = "gzip";
const GZIP_POSTFIX: &str = ".gz";

fn pkg_route(compressed_pkg: bool, pkg_dir: &'static str) -> BoxedFilter<(impl Reply,)> {
    let pkg_dir = Path::new(pkg_dir);

    path("pkg")
        .and(path::tail())
        .and(warp::header::optional(ACCEPT_ENCODING.as_str()))
        .and_then(
            move |file: path::Tail, accept_encoding: Option<String>| async move {
                let mut file = file.as_str().to_owned();
                let mime = mime_guess::from_path(&file).first_or_octet_stream();

                let mut headers = HeaderMap::new();
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_str(&mime.to_string()).unwrap(),
                );

                if compressed_pkg {
                    if let Some(accept_encoding) = accept_encoding {
                        let encodings = accept_encoding.split(", ").collect::<BTreeSet<_>>();
                        if encodings.contains(BROTLI_ID) {
                            file.push_str(BROTLI_POSTFIX);
                            headers.insert(CONTENT_ENCODING, HeaderValue::from_static(BROTLI_ID));
                        } else if encodings.contains(GZIP_ID) {
                            file.push_str(GZIP_POSTFIX);
                            headers.insert(CONTENT_ENCODING, HeaderValue::from_static(GZIP_ID));
                        }
                    }
                }
                let file = pkg_dir.join(file);
                let mut file = match tokio::fs::File::open(file).await {
                    Ok(file) => file,
                    Err(_) => return Err(warp::reject::not_found()),
                };
                let mut contents = vec![];
                file.read_to_end(&mut contents).await.unwrap();
                headers.insert(
                    CONTENT_LENGTH,
                    HeaderValue::from_str(&contents.len().to_string()).unwrap(),
                );

                let mut response = http::Response::new(contents);
                *response.headers_mut() = headers;
                Ok::<_, warp::Rejection>(response)
            },
        )
        .boxed()
}

fn html(
    title: &str,
    backend_build_id: u128,
    frontend_build_id: u128,
    append_to_head: &str,
    body_content: &str,
) -> String {
    format!(
        r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
      <meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
      <title>{title}</title>
      <link rel="preload" href="/pkg/frontend_bg_{frontend_build_id}.wasm" as="fetch" type="application/wasm" crossorigin>
      <link rel="modulepreload" href="/pkg/frontend_{frontend_build_id}.js" crossorigin>
      {append_to_head}
    </head>

    <body>
      {html_debug_info}
      {body_content}

      <script type="text/javascript">
        {reconnecting_event_source}
        var uri = location.protocol + '//' + location.host + '/sse';
        var sse = new ReconnectingEventSource(uri);
        var backendBuildId = null;
        sse.addEventListener("backend_build_id", function(msg) {{
            var newBackendBuildId = msg.data;
            if(backendBuildId === null) {{
                backendBuildId = newBackendBuildId;
            }} else if(backendBuildId !== newBackendBuildId) {{
                sse.close();
                location.reload();
            }}
          }});
        sse.addEventListener("reload", function(msg) {{
          sse.close();
          location.reload();
        }});
      </script>

      <script type="module">
        import init from '/pkg/frontend_{frontend_build_id}.js';
        init('/pkg/frontend_bg_{frontend_build_id}.wasm');
      </script>
    </body>
    
    </html>"#,
        title = title,
        html_debug_info = html_debug_info(backend_build_id),
        body_content = body_content,
        reconnecting_event_source = include_str!("../js/ReconnectingEventSource.min.js"),
        frontend_build_id = frontend_build_id.to_string(),
        append_to_head = append_to_head
    )
}

fn html_debug_info(_backend_build_id: u128) -> String {
    String::new()
    // format!("<h1>MoonZoon is running!</h1>
    //     <h2>Backend build id: {backend_build_id}</h2>
    //     <h2>Random id: {random_id}</h2>",
    //     backend_build_id = backend_build_id.to_string(),
    //     random_id = Uuid::new_v4().to_string()
    // )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pkg_route {

        use super::*;
        use const_format::concatcp;

        const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
        const FIXTURES_DIR: &str = concatcp!(MANIFEST_DIR, "/tests/fixtures/");

        #[tokio::test]
        async fn uncompressed() {
            let css_content = include_str!("../tests/fixtures/index.css");
            let filter = pkg_route(true, FIXTURES_DIR);
            let res = warp::test::request()
                .path("/pkg/index.css")
                .reply(&filter)
                .await;
            assert_eq!(res.status(), 200);
            assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
            assert_eq!(res.into_body(), css_content);
        }

        #[tokio::test]
        async fn brotli_compressed() {
            let css_content = include_bytes!("../tests/fixtures/index.css.br");
            let filter = pkg_route(true, FIXTURES_DIR);
            let res = warp::test::request()
                .header(ACCEPT_ENCODING, "br")
                .path("/pkg/index.css")
                .reply(&filter)
                .await;
            assert_eq!(res.status(), 200);
            assert_eq!(res.headers()[CONTENT_ENCODING], "br");
            assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
            assert_eq!(res.into_body().as_ref(), css_content);
        }

        #[tokio::test]
        async fn gzip_compressed() {
            let css_content = include_bytes!("../tests/fixtures/index.css.gz");
            let filter = pkg_route(true, FIXTURES_DIR);
            let res = warp::test::request()
                .header(ACCEPT_ENCODING, "gzip")
                .path("/pkg/index.css")
                .reply(&filter)
                .await;
            assert_eq!(res.status(), 200);
            assert_eq!(res.headers()[CONTENT_ENCODING], "gzip");
            assert_eq!(res.headers()[CONTENT_TYPE], "text/css");
            assert_eq!(res.into_body().as_ref(), css_content);
        }
    }
}
