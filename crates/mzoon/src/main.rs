use structopt::StructOpt;
use std::process::{Command, Stdio, Child};
use std::fs;
use serde::Deserialize;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent };
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread::{self, JoinHandle};

#[derive(Debug, StructOpt)]
enum Opt  {
    New { project_name: String },
    Start { 
        #[structopt(short, long)]
        release: bool
    },
}

// Run from example with build:  cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start
// Run from example:  ../../crates/mzoon/target/debug/mzoon start

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {},
        Opt::Start { release } => {
            let config = load_config();
            check_wasm_pack();
            
            let frontend_watcher_handle = start_frontend_watcher(config.watch.frontend.clone(), release);
            let backend_watcher_handle = start_backend_watcher(config.watch.backend.clone(), release);
            
            frontend_watcher_handle.join().unwrap();
            backend_watcher_handle.join().unwrap();
        },
    }
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
                        build_frontend(release);
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

fn load_config() -> Config {
    let toml = fs::read_to_string("MoonZoon.toml").unwrap();
    toml::from_str(&toml).unwrap()
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

fn build_frontend(release: bool) {
    let mut args = vec![
        "--log-level", "warn", "build", "frontend", "--target", "web", "--no-typescript",
    ];
    if !release {
        args.push("--dev");
    }
    Command::new("wasm-pack")
        .args(&args)
        .spawn()
        .unwrap();
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
