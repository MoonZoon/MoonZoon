use anyhow::{Context, Result};
use tokio::{spawn, task::JoinHandle, time::Duration};
use parking_lot::Mutex;
use std::sync::Arc;
use std::process::Child;
use crate::project_watcher::ProjectWatcher;
use crate::config::Config;
use crate::backend::{build_backend, run_backend};

pub struct BackendWatcher {
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl BackendWatcher {
    pub async fn start(config: &Config, release: bool, debounce_time: Duration, server: Option<Child>) -> Result<Self> {
        let (watcher, mut debounced_receiver) = ProjectWatcher::start(&config.watch.backend, debounce_time)
                .await
                .context("Failed to start the backend project watcher")?;
                
        let https = config.https;
        
        let task = spawn(async move {
            let mut build_task = None::<JoinHandle<()>>;
            let server = Arc::new(Mutex::new(server));
    
            while debounced_receiver.recv().await.is_some() {
                println!("Build backend");
                if let Some(build_task) = build_task.take() {
                    build_task.abort();
                }
                if let Some(mut server) = server.lock().take() {
                    let _ = server.kill();
                }
                let server = Arc::clone(&server);
                build_task = Some(spawn(async move {
                    match build_backend(release, https).await {
                        Ok(()) => {
                            match run_backend(release) {
                                Ok(backend) => { 
                                    server.lock().replace(backend);
                                },
                                Err(error) => {
                                    eprintln!("{}", error);
                                }
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
            task
        })
    }

    pub async fn stop(self) -> Result<()> {
        self.watcher.stop().await?;
        self.task.await??;
        Ok(())
    }
}
