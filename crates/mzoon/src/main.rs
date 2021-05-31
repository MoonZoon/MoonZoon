use std::env;
use structopt::StructOpt;
use anyhow::Result;
use tokio::{signal, select};
use tokio::time::Duration;
use std::process::Child;

mod config;
mod frontend;
mod backend;

use config::*;
use frontend::*;
use backend::*;

#[derive(Debug, StructOpt)]
enum Opt {
    New {
        project_name: String,
    },
    Start {
        #[structopt(short, long)]
        release: bool,
        // #[structopt(short, long)]
        // open: bool
    },
    Build {
        #[structopt(short, long)]
        release: bool,
    },
}

// Run from example:  cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {}
        Opt::Start { release } => {
            let config = Config::load_from_moonzoon_toml().await?;
            set_env_vars(&config, release);

            check_wasm_pack()?;

            if config.https {
                generate_certificate_if_not_present().await?;
            }

            let debounce_time = Duration::from_millis(100);

            if let Err(error) = build_frontend(release, config.cache_busting).await {
                eprintln!("{}", error);
            }
            let frontend_watcher_handle = start_frontend_watcher(&config, release, debounce_time);
            
            let mut server = None::<Child>;
            if let Err(error) = build_backend(release).await {
                eprintln!("{}", error);
            } else {
                match run_backend(release) {
                    Ok(backend) => {
                        // if open {
                        //     open = false;
                        //     let address = "https://127.0.0.1:8443";
                        //     println!("Open {} in your default web browser", "https://127.0.0.1:8443");
                        //     open::that(address).unwrap();
                        // }
                        server = Some(backend)
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
            let backend_watcher_handle = start_backend_watcher(&config, release, debounce_time, server);

            select! {
                result = signal::ctrl_c() => result?,
                result = frontend_watcher_handle => result??,
                result = backend_watcher_handle => result??,
            }
            println!("mzoon shut down");
        }
        Opt::Build { release } => {
            let config = Config::load_from_moonzoon_toml().await?;
            set_env_vars(&config, release);

            check_wasm_pack()?;

            if config.https {
                generate_certificate_if_not_present().await?;
            }

            build_frontend(release, config.cache_busting).await?;
            build_backend(release).await?;
        }
    }
    Ok(())
}

fn set_env_vars(config: &Config, release: bool) {
    // port = 8443
    env::set_var("PORT", config.port.to_string());
    // https = true
    env::set_var("HTTPS", config.https.to_string());

    // [redirect]
    // port = 8080
    env::set_var("REDIRECT_PORT", config.redirect.port.to_string());
    // enabled = true
    env::set_var("REDIRECT_ENABLED", config.redirect.enabled.to_string());

    env::set_var("COMPRESSED_PKG", release.to_string());
}
