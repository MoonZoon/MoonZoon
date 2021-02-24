use std::future::Future;
use std::error::Error;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::task;
use tokio::sync::oneshot;
use tokio::signal;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::http;
use warp::sse::Event;

pub struct Frontend {
    title: String,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            title: String::new()
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

pub struct UpMsgRequest {

}

#[macro_export]
macro_rules! start {
    ($init:expr, $frontend:expr, $up_msg_handler:expr) => {
        $crate::start($init, $frontend, $up_msg_handler);
    };
}

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
    let rt  = Runtime::new()?;
    rt.block_on(async move {
        let sse_sender = Arc::new(Mutex::new(None::<mpsc::UnboundedSender<Result<Event, Infallible>>>));
        let sse_sender = warp::any().map(move || sse_sender.clone());

        init().await;

        let api = warp::post().and(warp::path("api"));

        let up_msg_handler_route = api
            .and(warp::path("up_msg_handler"))
            .and_then(move || async move {
                up_msg_handler(UpMsgRequest {}).await;
                Ok::<_, warp::Rejection>(http::StatusCode::OK)
            });

        let reload = api
            .and(warp::path("reload"))
            .and(sse_sender.clone())
            .map(|sse_sender: Arc<Mutex<Option<mpsc::UnboundedSender<Result<Event, Infallible>>>>>| {
                let mut sse_sender = sse_sender.lock().unwrap();
                if let Some(sse_sender) = sse_sender.as_mut() {
                    sse_sender.send(Ok(Event::default().event("reload").data(""))).unwrap();
                }
                http::StatusCode::OK
            });

        let sse = warp::path!("sse")
            .and(sse_sender)
            .map(|shared_sse_sender: Arc<Mutex<Option<mpsc::UnboundedSender<Result<Event, Infallible>>>>>| {
                let (sse_sender, reload_event_receiver) = mpsc::unbounded_channel();
                *shared_sse_sender.lock().unwrap() = Some(sse_sender);
                let reload_event_stream = UnboundedReceiverStream::<Result<Event, Infallible>>::new(reload_event_receiver);
                warp::sse::reply(warp::sse::keep_alive().stream(reload_event_stream))
            });

        let pkg_route = warp::path("pkg").and(warp::fs::dir("./frontend/pkg/"));

        let frontend_route = warp::get().and_then(move || async move {
            let frontend = frontend().await;
            Ok::<_, warp::Rejection>(warp::reply::html(html(&frontend.title)))
        });
        
        let routes = up_msg_handler_route
            .or(reload)
            .or(sse)
            .or(pkg_route)
            .or(frontend_route);

        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let (_, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async {
                shutdown_receiver.await.ok();
            });
        task::spawn(server);
        signal::ctrl_c().await.unwrap();
        let _ = shutdown_sender.send(());
    });
    Ok(())
}

fn html(title: &str) -> String {
    format!(r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
      <meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no" />
      <title>{title}</title>
    </head>

    <body>
      <h1>Moon! {:#?}</h1>
      <section id="app"></section>

      <script type="text/javascript">
        var uri = 'http://' + location.host + '/sse';
        var sse = new EventSource(uri);
        sse.addEventListener("reload", function(msg) {{
            sse.close();
            location.reload();
        }});
      </script>

      <script type="module">
        import init from '/pkg/frontend.js';
        init('/pkg/frontend_bg.wasm');
      </script>
    </body>
    
    </html>"#, std::time::SystemTime::now(), title = title)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
