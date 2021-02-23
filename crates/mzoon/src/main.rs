use structopt::StructOpt;
use std::process::{Command, Stdio};
use std::fs;
use serde::Deserialize;

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

    let config;

    match opt {
        Opt::New { .. } => {},
        Opt::Start { release } => {
            config = load_config();
            check_wasm_pack();
            build_frontend(release);
            build_and_run_backend(release)
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

fn build_and_run_backend(release: bool) {
    let mut args = vec![
        "run", "--package", "backend",
    ];
    if release {
        args.push("--release");
    }
    Command::new("cargo")
        .args(&args)
        .spawn()
        .unwrap();
}
