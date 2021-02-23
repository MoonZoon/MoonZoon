use structopt::StructOpt;
use std::process::{Command, Stdio};

#[derive(Debug, StructOpt)]
enum Opt  {
    New { project_name: String },
    Start,
}

// Run from example with build:  cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start
// Run from example:  ../../crates/mzoon/target/debug/mzoon start

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {},
        Opt::Start => {
            Command::new("cargo")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .args(&["run", "--package", "backend"])
                .output()
                .unwrap();
        },
    }
}
