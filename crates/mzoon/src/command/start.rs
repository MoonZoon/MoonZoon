use anyhow::{Context, Error};
use tokio::{signal, time::Duration, join, process::Child};
use fehler::throws;
use crate::watcher::{FrontendWatcher, BackendWatcher};
use crate::set_env_vars::set_env_vars;
use crate::config::Config;
use crate::build_frontend::build_frontend;
use crate::build_backend::build_backend;
use crate::run_backend::run_backend;

const DEBOUNCE_TIME: Duration = Duration::from_millis(100);

#[throws]
pub async fn start(release: bool, open: bool) {
    let config = Config::load_from_moonzoon_toml().await?;
    set_env_vars(&config, release);

    let frontend_watcher = build_and_watch_frontend(&config, release).await?;
    let backend_watcher = build_run_and_watch_backend(&config, release, open).await?;

    signal::ctrl_c().await?;
    println!("Stopping watchers...");
    let _ = join!(
        frontend_watcher.stop(),
        backend_watcher.stop(),
    );
    println!("Watchers stopped");
}

#[throws]
async fn build_and_watch_frontend(config: &Config, release: bool) -> FrontendWatcher {
    if let Err(error) = build_frontend(release, config.cache_busting).await {
        eprintln!("{}", error);
    }
    FrontendWatcher::start(&config, release, DEBOUNCE_TIME).await?
}

#[throws]
async fn build_run_and_watch_backend(config: &Config, release: bool, open: bool) -> BackendWatcher {
    let server = build_and_run_backend(config, release).await;
    if open && server.is_some() {
        open_in_browser(config)?;
    }
    BackendWatcher::start(&config, release, DEBOUNCE_TIME, server).await?
}

#[throws(as Option)]
async fn build_and_run_backend(config: &Config, release: bool) -> Child {
    if let Err(error) = build_backend(release, config.https).await {
        eprintln!("{}", error);
        None?;
    }
    match run_backend(release) {
        Err(error) => {
            eprintln!("{}", error);
            None?
        }
        Ok(server) => server,
    }
}

#[throws]
fn open_in_browser(config: &Config) {
    let url = format!(
        "{protocol}://localhost:{port}", 
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    );
    println!("Open {} in the default web browser", url);
    open::that(url).context("Failed to open the URL in the browser")?;
} 
