use anyhow::{Context, Result};
use tokio::{spawn, task::JoinHandle, time::Duration};
use std::sync::Arc;
use crate::project_watcher::ProjectWatcher;
use crate::config::Config;
use crate::frontend::build_frontend;

pub struct FrontendWatcher {
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl FrontendWatcher {
    pub async fn start(config: &Config, release: bool, debounce_time: Duration) -> Result<Self> {
        let (watcher, mut debounced_receiver) = ProjectWatcher::start(&config.watch.frontend, debounce_time)
                .await
                .context("Failed to start the frontend project watcher")?;

        let reload_url = Arc::new(format!(
            "{protocol}://localhost:{port}/_api/reload",
            protocol = if config.https { "https" } else { "http" },
            port = config.port
        ));
        let cache_busting = config.cache_busting;

        let task = spawn(async move {
            let mut build_task = None::<JoinHandle<()>>;
            while debounced_receiver.recv().await.is_some() {
                println!("Build frontend");
                if let Some(build_task) = build_task.take() {
                    build_task.abort();
                }
                let reload_url = Arc::clone(&reload_url);
                build_task = Some(spawn(async move {
                    match build_frontend(release, cache_busting).await {
                        Ok(()) => {
                            println!("Reload frontend");
                            let response = attohttpc::post(reload_url.as_str())
                                .danger_accept_invalid_certs(true)
                                .send();
                            if let Err(error) = response {
                                eprintln!("Failed to send the frontend reload request: {:#?}", error);
                            }
                        }
                        Err(error) => {
                            eprintln!("{}", error);
                        }
                    }
                }));
            }
            Ok(())
        });
        
        Ok(Self {
            watcher,
            task,
        })
    }

    pub async fn stop(self) -> Result<()> {
        self.watcher.stop().await?;
        self.task.await??;
        Ok(())
    }
}

