use structopt::StructOpt;
use std::process::{Command, Stdio, Child, exit};
use std::fs;
use serde::Deserialize;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent };
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::path::Path;

#[derive(Debug, StructOpt)]
enum Opt  {
    New { project_name: String },
    Start { 
        #[structopt(short, long)]
        release: bool
    },
}

// Run from example:  cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start

fn main() {
    ctrlc::set_handler(|| exit(0)).unwrap();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {},
        Opt::Start { release } => {
            let config = load_config();
            check_wasm_pack();
            generate_certificate();    
            
            let frontend_watcher_handle = start_frontend_watcher(config.watch.frontend.clone(), release);
            let backend_watcher_handle = start_backend_watcher(config.watch.backend.clone(), release);
            
            frontend_watcher_handle.join().unwrap();
            backend_watcher_handle.join().unwrap();
        },
    }
}

fn load_config() -> Config {
    let toml = fs::read_to_string("MoonZoon.toml").unwrap();
    toml::from_str(&toml).unwrap()
}

#[derive(Debug, Deserialize)]
struct Config {
    watch: Watch
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
        _ => panic!("Cannot find `wasm-pack`! Please install it by `cargo install wasm-pack`"),
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
    let certificate = rcgen::generate_simple_self_signed(domains).unwrap();
    let public_pem = certificate.serialize_pem().unwrap();
    let private_pem = certificate.serialize_private_key_pem();
    fs::write(public_pem_path, public_pem).unwrap();
    fs::write(private_pem_path, private_pem).unwrap();
}

fn start_frontend_watcher(paths: Vec<String>, release: bool) -> JoinHandle<()> {
    thread::spawn(move || {
        let (watcher_sender, watcher_receiver) = channel();
        let mut watcher = watcher(watcher_sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }
        build_frontend(release);
        loop {
            match watcher_receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent:: NoticeRemove(_) => (),
                    _ => {
                        println!("Build frontend");
                        if build_frontend(release) {
                            println!("Reload frontend");
                            reqwest::blocking::Client::new()
                                .post("http://127.0.0.1:8080/api/reload")
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

fn start_backend_watcher(paths: Vec<String>, release: bool) -> JoinHandle<()> {
    thread::spawn(move || {
        let (watcher_sender, watcher_receiver) = channel();
        let mut watcher = watcher(watcher_sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }

        let (server_rebuild_run_sender, server_rebuild_run_receiver) = channel();
        thread::spawn(move || {
            loop {
                let mut cargo_and_server_process = build_and_run_backend(release);
                server_rebuild_run_receiver.recv().unwrap();
                let _ = cargo_and_server_process.kill();
            }
        });

        loop {
            match watcher_receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent:: NoticeRemove(_) => (),
                    _ => {
                        println!("Build backend");
                        server_rebuild_run_sender.send(()).unwrap();
                    }
                },
                Err(error) => println!("watch backend error: {:?}", error),
            }
        }
    })
}

fn build_frontend(release: bool) -> bool {
    let mut args = vec![
        "--log-level", "warn", "build", "frontend", "--target", "web", "--no-typescript",
    ];
    if !release {
        args.push("--dev");
    }
    Command::new("wasm-pack")
        .args(&args)
        .status()
        .unwrap()
        .success()
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
