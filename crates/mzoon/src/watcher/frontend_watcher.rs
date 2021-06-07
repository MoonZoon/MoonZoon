use super::project_watcher::ProjectWatcher;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use anyhow::{Context, Error, Result};
use fehler::throws;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::{spawn, task::JoinHandle, time::Duration};

pub struct FrontendWatcher {
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl FrontendWatcher {
    #[throws]
    pub async fn start(config: &Config, release: bool, debounce_time: Duration) -> Self {
        let (watcher, debounced_receiver) =
            ProjectWatcher::start(&config.watch.frontend, debounce_time)
                .await
                .context("Failed to start the frontend project watcher")?;

        let reload_url = Arc::new(format!(
            "{protocol}://localhost:{port}/_api/reload",
            protocol = if config.https { "https" } else { "http" },
            port = config.port
        ));

        Self {
            watcher,
            task: spawn(on_change(
                debounced_receiver,
                reload_url,
                release,
                config.cache_busting,
            )),
        }
    }

    #[throws]
    pub async fn stop(self) {
        self.watcher.stop().await?;
        self.task.await??;
    }
}

#[throws]
async fn on_change(
    mut receiver: UnboundedReceiver<()>,
    reload_url: Arc<String>,
    release: bool,
    cache_busting: bool,
) {
    let mut build_task = None::<JoinHandle<()>>;

    while receiver.recv().await.is_some() {
        if let Some(build_task) = build_task.take() {
            build_task.abort();
        }
        build_task = Some(spawn(build_and_reload(
            Arc::clone(&reload_url),
            release,
            cache_busting,
        )));
    }

    if let Some(build_task) = build_task.take() {
        build_task.abort();
    }
}

async fn build_and_reload(reload_url: Arc<String>, release: bool, cache_busting: bool) {
    if let Err(error) = build_frontend(release, cache_busting).await {
        return eprintln!("{}", error);
    }
    println!("Reload frontend");
    let response = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(reload_url.as_str())
        .send()
        .await;
    if let Err(error) = response {
        eprintln!("Failed to send the frontend reload request: {:#?}", error);
    }
}
