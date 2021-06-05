use anyhow::{Context, Error};
use tokio::{signal, time::Duration, join, process::Child};
use fehler::throws;
use crate::watcher::{FrontendWatcher, BackendWatcher};
use crate::set_env_vars::set_env_vars;
use crate::config::Config;
use crate::build_frontend::build_frontend;
use crate::build_backend::build_backend;
use crate::run_backend::run_backend;

#[throws]
pub async fn start(release: bool, open: bool) {
    let config = Config::load_from_moonzoon_toml().await?;
    set_env_vars(&config, release);

    let debounce_time = Duration::from_millis(100);

    if let Err(error) = build_frontend(release, config.cache_busting).await {
        eprintln!("{}", error);
    }
    let frontend_watcher = FrontendWatcher::start(&config, release, debounce_time).await?;
    
    let mut server = None::<Child>;
    if let Err(error) = build_backend(release, config.https).await {
        eprintln!("{}", error);
    } else {
        match run_backend(release) {
            Ok(backend) => {
                if open {
                    let url = format!(
                        "{protocol}://localhost:{port}", 
                        protocol = if config.https { "https" } else { "http" },
                        port = config.port
                    );
                    println!("Open {} in the default web browser", url);
                    open::that(url).context("Failed to open the URL in the browser")?;
                }
                server = Some(backend)
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    let backend_watcher = BackendWatcher::start(&config, release, debounce_time, server).await?;

    signal::ctrl_c().await?;
    println!("Stopping watchers...");
    let _ = join!(
        frontend_watcher.stop(),
        backend_watcher.stop(),
    );
    println!("Watchers stopped");
}
