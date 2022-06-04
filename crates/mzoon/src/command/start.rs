use crate::build_backend::build_backend;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use crate::run_backend::run_backend;
use crate::set_env_vars::set_env_vars;
use crate::watcher::{BackendWatcher, FrontendWatcher};
use crate::BuildMode;
use anyhow::{Context, Error};
use fehler::throws;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::{join, process::Child, signal, time::Duration};

const DEBOUNCE_TIME: Duration = Duration::from_millis(100);

#[throws]
pub async fn start(build_mode: BuildMode, open: bool) {
    let config = Config::load_from_moonzoon_tomls().await?;
    set_env_vars(&config, build_mode);

    let server = Arc::new(Mutex::new(None));

    let frontend_watcher = build_and_watch_frontend(&config, build_mode).await?;
    let backend_watcher =
        build_run_and_watch_backend(&config, build_mode, open, Arc::clone(&server)).await?;

    signal::ctrl_c().await?;

    println!("Stopping watchers...");
    let _ = join!(frontend_watcher.stop(), backend_watcher.stop(),);
    println!("Watchers stopped");

    if let Some(server) = server.lock().as_mut() {
        println!("Stopping Moon server...");
        let _ = server.wait().await;
        println!("Moon stopped");
    }

    // @TODO resolve the problem with killing the `start` task when it's started by `makers`
    // https://github.com/sagiegurari/cargo-make/issues/374#issuecomment-1094366124
}

#[throws]
async fn build_and_watch_frontend(config: &Config, build_mode: BuildMode) -> FrontendWatcher {
    if let Err(error) = build_frontend(build_mode, config.cache_busting, false).await {
        eprintln!("{error:#}");
    }
    FrontendWatcher::start(&config, build_mode, DEBOUNCE_TIME).await?
}

#[throws]
async fn build_run_and_watch_backend(
    config: &Config,
    build_mode: BuildMode,
    open: bool,
    server: Arc<Mutex<Option<Child>>>,
) -> BackendWatcher {
    build_and_run_backend(config, build_mode, &server).await;
    if open {
        open_in_browser(config)?;
    }
    BackendWatcher::start(&config, build_mode, DEBOUNCE_TIME, server).await?
}

async fn build_and_run_backend(
    config: &Config,
    build_mode: BuildMode,
    server: &Mutex<Option<Child>>,
) {
    if let Err(error) = build_backend(build_mode, config.https).await {
        eprintln!("{error:#}");
        return;
    }
    match run_backend(build_mode) {
        Err(error) => {
            eprintln!("{error:#}");
        }
        Ok(server_process) => {
            *server.lock() = Some(server_process);
        }
    }
}

#[throws]
fn open_in_browser(config: &Config) {
    let url = format!(
        "{protocol}://localhost:{port}",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    );
    println!("Open {url} in the default web browser");
    open::that(url).context("Failed to open the URL in the browser")?;
}
