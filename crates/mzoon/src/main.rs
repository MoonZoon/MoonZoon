use notify::DebouncedEvent;
use std::env;
use std::sync::mpsc::channel;
use structopt::StructOpt;

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
async fn main() {
    let (frontend_sender, frontend_receiver) = channel();
    let (backend_sender, backend_receiver) = channel();
    ctrlc::set_handler({
        let frontend_sender = frontend_sender.clone();
        let backend_sender = backend_sender.clone();
        move || {
            println!("Shut down MoonZoon CLI");
            let event = || DebouncedEvent::Error(notify::Error::Generic("ctrl-c".to_owned()), None);
            frontend_sender.send(event()).unwrap();
            backend_sender.send(event()).unwrap();
        }
    })
    .unwrap();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {}
        Opt::Start { release } => {
            let config = load_config();
            set_env_vars(&config, release);

            check_wasm_pack();
            if config.https {
                generate_certificate();
            }

            let (frontend_build_finished_sender, frontend_build_finished_receiver) = channel();
            let frontend_watcher_handle = start_frontend_watcher(
                config.watch.frontend.clone(),
                release,
                frontend_sender,
                frontend_receiver,
                frontend_build_finished_sender,
                &config,
            );
            // @TODO parallel build instead of waiting (server has to be started after FE build!)
            // `recv` fails if the sender is dropped because of fail in `start_frontend_watcher`
            let _ = frontend_build_finished_receiver.recv();

            let backend_watcher_handle = start_backend_watcher(
                config.watch.backend.clone(),
                release,
                // open,
                backend_sender,
                backend_receiver,
            );
            frontend_watcher_handle.join().unwrap();
            backend_watcher_handle.join().unwrap();
        }
        Opt::Build { release } => {
            let config = load_config();
            set_env_vars(&config, release);

            check_wasm_pack();
            if config.https {
                generate_certificate();
            }

            if !build_frontend(release, config.cache_busting) {
                panic!("Build frontend failed!");
            }

            if !build_backend(release) {
                panic!("Build backend failed!");
            }
        }
    }
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
