use structopt::StructOpt;
use anyhow::Error;
use fehler::throws;

mod config;
mod frontend;
mod backend;
mod watcher;
mod set_env_vars;
mod command;
mod wasm_pack;
mod helper;

#[derive(Debug, StructOpt)]
enum Opt {
    New {
        project_name: String,
        #[structopt(short, long)]
        here: bool
    },
    Start {
        #[structopt(short, long)]
        release: bool,
        #[structopt(short, long)]
        open: bool
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
