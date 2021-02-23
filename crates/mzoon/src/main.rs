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
            let mut wasm_pack_args = vec![
                "--log-level", "warn", "build", "frontend", "--target", "web", "--no-typescript",
            ];
            if !release {
                wasm_pack_args.push("--dev");
            }
            Command::new("wasm-pack")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&wasm_pack_args)
                .output()
                .unwrap();

            let mut cargo_args = vec![
                "run", "--package", "backend",
            ];
            if release {
                cargo_args.push("--release");
            }
            Command::new("cargo")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&cargo_args)
                .output()
                .unwrap();
        },
    }
}
