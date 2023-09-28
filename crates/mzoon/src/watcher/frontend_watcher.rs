use super::project_watcher::ProjectWatcher;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use crate::BuildMode;
use anyhow::{Context, Error, Result};
use fehler::throws;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedReceiver, watch};
use tokio::{
    spawn,
    task::JoinHandle,
    time::{sleep, Duration},
};

pub struct FrontendWatcher {
    #[allow(dead_code)]
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl FrontendWatcher {
    #[throws]
    pub async fn start(config: &Config, build_mode: BuildMode, debounce_time: Duration) -> Self {
        let (watcher, debounced_receiver) =
            ProjectWatcher::start(&config.watch.frontend, debounce_time)
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
                build_mode,
                config.cache_busting,
                config.frontend_multithreading == Some(true),
            )),
        }
    }

    #[throws]
    pub async fn stop(self) {
        drop(self.watcher);
        self.task.await??;
    }
}

#[throws]
async fn on_change(
    mut receiver: UnboundedReceiver<()>,
    reload_url: Arc<String>,
    build_mode: BuildMode,
    cache_busting: bool,
    frontend_multithreading: bool,
) {
    let mut build_task = None::<JoinHandle<()>>;
    let mut compilation_killer_sender = None::<watch::Sender<()>>;

    while receiver.recv().await.is_some() {
        if let Some(compilation_killer_sender) = compilation_killer_sender.take() {
            drop(compilation_killer_sender);
            // `sleep` / next tick is required to give the runtime chance to handle sender's drop
            // before calling `build_task.abort()` so the associated receivers will be notified
            // about the drop and then compilation processes will receive kill signals
            sleep(Duration::from_millis(0)).await;
        }
        if let Some(build_task) = build_task.take() {
            build_task.abort();
        }

        let (new_compilation_killer_sender, _) = watch::channel(());
        build_task = Some(spawn(build_and_reload(
            Arc::clone(&reload_url),
            build_mode,
            cache_busting,
            frontend_multithreading,
            Some(new_compilation_killer_sender.subscribe()),
        )));
        compilation_killer_sender = Some(new_compilation_killer_sender);
    }

    if let Some(compilation_killer_sender) = compilation_killer_sender.take() {
        drop(compilation_killer_sender);
        sleep(Duration::from_millis(0)).await;
    }
    if let Some(build_task) = build_task.take() {
        build_task.abort();
    }
}

async fn build_and_reload(
    reload_url: Arc<String>,
    build_mode: BuildMode,
    cache_busting: bool,
    frontend_multithreading: bool,
    compilation_killer: Option<watch::Receiver<()>>,
) {
    if let Err(error) = build_frontend(
        build_mode,
        cache_busting,
        false,
        frontend_multithreading,
        compilation_killer,
    )
    .await
    {
        return eprintln!("{error}");
    }
    if build_mode.is_release() {
        return println!("['Reload frontend' is deactivated in release mode]");
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
        eprintln!("Failed to send the frontend reload request: {:?}", error);
    }
}
