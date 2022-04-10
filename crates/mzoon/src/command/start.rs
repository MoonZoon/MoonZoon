use crate::build_backend::build_backend;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use crate::run_backend::run_backend;
use crate::set_env_vars::set_env_vars;
use crate::watcher::{BackendWatcher, FrontendWatcher};
use parking_lot::Mutex;
use std::sync::Arc;
use anyhow::{Context, Error};
use fehler::throws;
use tokio::{join, process::Child, signal, time::Duration};

const DEBOUNCE_TIME: Duration = Duration::from_millis(100);

#[throws]
pub async fn start(release: bool, open: bool) {
    let config = Config::load_from_moonzoon_tomls().await?;
    set_env_vars(&config, release);

    let server = Arc::new(Mutex::new(None));
    
    let frontend_watcher = build_and_watch_frontend(&config, release).await?;
    let backend_watcher = build_run_and_watch_backend(&config, release, open, Arc::clone(&server)).await?;

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
async fn build_and_watch_frontend(config: &Config, release: bool) -> FrontendWatcher {
    if let Err(error) = build_frontend(release, config.cache_busting).await {
        eprintln!("{}", error);
    }
    FrontendWatcher::start(&config, release, DEBOUNCE_TIME).await?
}

#[throws]
async fn build_run_and_watch_backend(config: &Config, release: bool, open: bool, server: Arc<Mutex<Option<Child>>>) -> BackendWatcher {
    build_and_run_backend(config, release, &server).await;
    if open {
        open_in_browser(config)?;
    }
    BackendWatcher::start(&config, release, DEBOUNCE_TIME, server).await?
}

async fn build_and_run_backend(config: &Config, release: bool, server: &Mutex<Option<Child>>) {
    if let Err(error) = build_backend(release, config.https).await {
        eprintln!("{}", error);
        return;
    }
    match run_backend(release) {
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
        Ok(server_process) => {
            *server.lock() = Some(server_process);
        },
    }
}

#[throws]
fn open_in_browser(config: &Config) {
    let url = server_url(config);
    println!("Open {} in the default web browser", url);
    open::that(url).context("Failed to open the URL in the browser")?;
}

fn server_url(config: &Config) -> String {
    let protocol = if config.https { "https" } else { "http" };
    let port = config.port;
    format!("{protocol}://localhost:{port}")
}
