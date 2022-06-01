use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;

mod build_backend;
mod build_frontend;
mod command;
mod config;
mod helper;
mod run_backend;
mod set_env_vars;
mod wasm_bindgen;
mod watcher;

#[derive(Debug, StructOpt)]
enum Opt {
    New {
        project_name: String,
        #[structopt(short, long)]
        here: bool,
    },
    Start {
        #[structopt(short, long)]
        release: bool,
        #[structopt(short, long)]
        open: bool,
    },
    Build {
        #[structopt(short, long)]
        release: bool,
    },
}

#[throws]
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { project_name, here } => command::new(project_name, here).await?,
        Opt::Start { release, open } => command::start(release, open).await?,
        Opt::Build { release } => command::build(release).await?,
    }
}
