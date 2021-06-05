use anyhow::{Context, Result, Error};
use tokio::{spawn, task::JoinHandle, time::Duration};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::process::Child;
use parking_lot::Mutex;
use std::sync::Arc;
use fehler::throws;
use super::project_watcher::ProjectWatcher;
use crate::config::Config;
use crate::build_backend::build_backend;
use crate::run_backend::run_backend;

pub struct BackendWatcher {
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl BackendWatcher {
    #[throws]
    pub async fn start(config: &Config, release: bool, debounce_time: Duration, server: Option<Child>) -> Self {
        let (watcher, debounced_receiver) = ProjectWatcher::start(&config.watch.backend, debounce_time)
            .await
            .context("Failed to start the backend project watcher")?;
        Self {
            watcher, 
            task: spawn(on_change(debounced_receiver, release, config.https, server))
        }
    }

    #[throws]
    pub async fn stop(self) {
        self.watcher.stop().await?;
        self.task.await??;
    }
}

#[throws]
async fn on_change(mut receiver: UnboundedReceiver<()>, release: bool, https: bool, server: Option<Child>) {
    let mut build_task = None::<JoinHandle<()>>;
    let server = Arc::new(Mutex::new(server));

    while receiver.recv().await.is_some() {
        if let Some(build_task) = build_task.take() {
            build_task.abort();
        }
        if let Some(mut server) = server.lock().take() {
            let _ = server.kill();
        }
        build_task = Some(spawn(build_and_run(Arc::clone(&server), release, https)));
    }
    
    if let Some(build_task) = build_task.take() {
        build_task.abort();
    }
} 

async fn build_and_run(server: Arc<Mutex<Option<Child>>>, release: bool, https: bool) {
    if let Err(error) = build_backend(release, https).await {
        return eprintln!("{}", error);
    }
    match run_backend(release) {
        Ok(backend) => *server.lock() = Some(backend),
        Err(error) => eprintln!("{}", error),
    }
}

