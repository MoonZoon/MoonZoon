use structopt::StructOpt;
use std::process::{Command, Stdio};

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
            check_wasm_pack();
            wasm_pack_build(release);
            cargo_run(release)
        },
    }
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

fn wasm_pack_build(release: bool) {
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

fn cargo_run(release: bool) {
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
