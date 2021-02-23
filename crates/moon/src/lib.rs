use std::future::Future;
use std::error::Error;
use tokio::runtime::Runtime;
use tokio::task;
use tokio::sync::oneshot;
use tokio::signal;
use warp::Filter;
use warp::http;

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
        init().await;

        let api = warp::post().and(warp::path("api"));

        let up_msg_handler_route = api
            .and(warp::path("up_msg_handler"))
            .and_then(move || async move {
                up_msg_handler(UpMsgRequest {}).await;
                Ok::<_, warp::Rejection>(http::StatusCode::OK)
            });

        let pkg_route = warp::path("pkg").and(warp::fs::dir("./frontend/pkg/"));

        let frontend_route = warp::get().and_then(move || async move {
            let frontend = frontend().await;
            Ok::<_, warp::Rejection>(warp::reply::html(html(&frontend.title)))
        });
        
        let routes = up_msg_handler_route
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
      <h1>Moon!</h1>
      <section id="app"></section>
      <script type="module">
        import init from '/pkg/frontend.js';
        init('/pkg/frontend_bg.wasm');
      </script>
    </body>
    
    </html>"#, title = title)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
