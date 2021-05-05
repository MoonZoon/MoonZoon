use structopt::StructOpt;
use std::process::{Command, Stdio, Child};
use std::fs;
use serde::Deserialize;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent };
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::{self, JoinHandle};
use std::path::Path;
use uuid::Uuid;
use rcgen::{Certificate, CertificateParams};
use std::env;

#[derive(Debug, StructOpt)]
enum Opt  {
    New { project_name: String },
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

fn main() {
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
    }).unwrap();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {},
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
            frontend_build_finished_receiver.recv().unwrap();

            let backend_watcher_handle = start_backend_watcher(
                config.watch.backend.clone(), 
                release,
                // open,
                backend_sender,
                backend_receiver,
            );
            frontend_watcher_handle.join().unwrap();
            backend_watcher_handle.join().unwrap();
        },
        Opt::Build { release } => {
            let config = load_config();
            set_env_vars(&config, release);            

            check_wasm_pack();
            if config.https {
                generate_certificate();    
            }

            if !build_frontend(release) {
                panic!("Build frontend failed!");
            }

            if !build_backend(release) {
                panic!("Build backend failed!");
            }
        },
    }
}

fn load_config() -> Config {
    let toml = fs::read_to_string("MoonZoon.toml").unwrap();
    toml::from_str(&toml).unwrap()
}

fn set_env_vars(config: &Config, release: bool) {
    // port = 8443
    env::set_var("PORT", config.port.to_string());
    // https = true
    env::set_var("HTTPS", config.https.to_string());

    // [redirect_server]
    // port = 8080
    env::set_var("REDIRECT_SERVER__PORT", config.redirect_server.port.to_string());
    // enabled = true
    env::set_var("REDIRECT_SERVER__ENABLED", config.redirect_server.enabled.to_string());

    env::set_var("COMPRESSED_PKG", release.to_string());
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    https: bool,
    redirect_server: RedirectServer,
    watch: Watch,
}

#[derive(Debug, Deserialize)]
struct RedirectServer {
    port: u16, 
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct Watch {
    frontend: Vec<String>, 
    backend: Vec<String>,
}

fn check_wasm_pack() {
    let status = Command::new("wasm-pack")
        .args(&["-V"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => (),
        _ => panic!("Cannot find `wasm-pack`! Please install it by `cargo install wasm-pack` or download/install pre-built binaries into a globally available directory."),
    }
}

fn generate_certificate() {
    let public_pem_path = Path::new("backend/private/public.pem");
    let private_pem_path = Path::new("backend/private/private.pem");
    if public_pem_path.is_file() && private_pem_path.is_file() { 
        return;
    }
    println!("Generate TLS certificate");

    let domains = vec!["localhost".to_owned()];
    let mut params = CertificateParams::new(domains);

    // https://support.mozilla.org/en-US/kb/Certificate-contains-the-same-serial-number-as-another-certificate
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    params.serial_number = Some(since_the_epoch);

    let certificate = Certificate::from_params(params).unwrap();

    let public_pem = certificate.serialize_pem().unwrap();
    let private_pem = certificate.serialize_private_key_pem();
    fs::write(public_pem_path, public_pem).unwrap();
    fs::write(private_pem_path, private_pem).unwrap();
}

fn start_frontend_watcher(
    paths: Vec<String>, 
    release: bool, 
    sender: Sender<DebouncedEvent>, 
    receiver: Receiver<DebouncedEvent>,
    frontend_build_finished_sender: Sender<()>,
    config: &Config,
) -> JoinHandle<()> {
    let reload_url = format!(
        "{protocol}://localhost:{port}/api/reload",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    );

    thread::spawn(move || {
        let mut watcher = watcher(sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }
        build_frontend(release);
        frontend_build_finished_sender.send(()).unwrap();
        loop {
            match receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent:: NoticeRemove(_) => (),
                    DebouncedEvent::Error(notify::Error::Generic(error), _) if error == "ctrl-c" => break,
                    _ => {
                        println!("Build frontend");
                        if build_frontend(release) {
                            println!("Reload frontend");
                            attohttpc::post(&reload_url)
                                .danger_accept_invalid_certs(true)
                                .send()
                                .unwrap();
                        }
                    }
                },
                Err(error) => panic!("watch frontend error: {:?}", error),
            }
        }
    })
}

enum BackendCommand {
    Rebuild,
    Stop,
}

fn start_backend_watcher(
    paths: Vec<String>, 
    release: bool, 
    // open: bool,
    sender: Sender<DebouncedEvent>, 
    receiver: Receiver<DebouncedEvent>
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut watcher = watcher(sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }

        let (server_rebuild_run_sender, server_rebuild_run_receiver) = channel();
        let backend_handle = thread::spawn(move || {
            // let mut open = open;
            loop {
                // @TODO only on successful build
                generate_backend_build_id();
                let mut cargo_and_server_process = build_and_run_backend(release);
                // @TODO wait for (successful) build
                // if open {
                //     open = false;
                //     let address = "https://127.0.0.1:8443";
                //     println!("Open {} in your default web browser", "https://127.0.0.1:8443");
                //     open::that(address).unwrap();
                // }
                let command = server_rebuild_run_receiver.recv().unwrap();
                match command {
                    BackendCommand::Rebuild => {
                        let _ = cargo_and_server_process.kill();
                    },
                    BackendCommand::Stop => { 
                        cargo_and_server_process.wait().unwrap();
                        break 
                    },
                }
            }
        });

        loop {
            match receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent:: NoticeRemove(_) => (),
                    DebouncedEvent::Error(notify::Error::Generic(error), _) if error == "ctrl-c" => {
                        server_rebuild_run_sender.send(BackendCommand::Stop).unwrap();
                        backend_handle.join().unwrap();
                        break
                    },
                    _ => {
                        println!("Build backend");
                        server_rebuild_run_sender.send(BackendCommand::Rebuild).unwrap();
                    }
                },
                Err(error) => println!("watch backend error: {:?}", error),
            }
        }
    })
}

fn generate_backend_build_id() {
    fs::write("backend/private/build_id", Uuid::new_v4().to_string()).unwrap();
}

fn build_frontend(release: bool) -> bool {
    let old_build_id = fs::read_to_string("frontend/pkg/build_id")
        .ok()
        .and_then(|uuid| uuid.parse::<Uuid>().ok());
    if let Some(old_build_id) = old_build_id {
        let old_wasm = format!("frontend/pkg/frontend_bg_{}.wasm", old_build_id);
        let old_js = format!("frontend/pkg/frontend_{}.js", old_build_id);
        let _ = fs::remove_file(old_wasm);
        let _ = fs::remove_file(old_js);
    }

    let mut args = vec![
        "--log-level", "warn", "build", "frontend", "--target", "web", "--no-typescript",
    ];
    if !release {
        args.push("--dev");
    }
    let success = Command::new("wasm-pack")
        .args(&args)
        .status()
        .unwrap()
        .success();
    if success {
        let build_id = Uuid::new_v4();
        fs::rename("frontend/pkg/frontend_bg.wasm", format!("frontend/pkg/frontend_bg_{}.wasm", build_id)).unwrap(); 
        fs::rename("frontend/pkg/frontend.js", format!("frontend/pkg/frontend_{}.js", build_id)).unwrap(); 
        fs::write("frontend/pkg/build_id", build_id.to_string()).unwrap();
    }    
    success
}

fn build_and_run_backend(release: bool) -> Child {
    let mut args = vec![
        "run", "--package", "backend",
    ];
    if release {
        args.push("--release");
    }
    Command::new("cargo")
        .args(&args)
        .spawn()
        .unwrap()
}

fn build_backend(release: bool) -> bool {
    let mut args = vec![
        "build", "--package", "backend",
    ];
    if release {
        args.push("--release");
    }
    let success = Command::new("cargo")
        .args(&args)
        .status()
        .unwrap()
        .success();
    if success {
        generate_backend_build_id();
    }
    success
}
