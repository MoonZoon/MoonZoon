use anyhow::Error;
use clap::Parser;
use fehler::throws;

mod build_backend;
mod build_frontend;
mod command;
mod config;
mod frontend_dist;
mod helper;
mod run_backend;
mod set_env_vars;
mod wasm_bindgen;
mod wasm_opt;
mod watcher;

/// MoonZoon CLI <http://MoonZoon.rs>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum Args {
    New {
        project_name: String,
        #[clap(short, long)]
        here: bool,
    },
    Start {
        #[clap(short, long)]
        release: bool,
        #[clap(short, long)]
        profiling: bool,
        #[clap(short, long)]
        open: bool,
    },
    Build {
        #[clap(short, long)]
        release: bool,
        #[clap(short, long)]
        profiling: bool,
        /// Prepare for frontend-only deploy; You can test it with https://crates.io/crates/microserver
        #[clap(short, long)]
        frontend_dist: bool,
        #[clap(arg_enum)]
        hosting: Option<Hosting>,
    },
}

#[derive(clap::ArgEnum, Clone, Copy, Debug)]
pub enum Hosting {
    Netlify,
}

#[derive(Debug, Copy, Clone)]
pub enum BuildMode {
    Dev,
    Profiling,
    Release,
}

impl BuildMode {
    fn new(release: bool, profiling: bool) -> Self {
        match (release, profiling) {
            (false, false) => Self::Dev,
            (true, false) => Self::Release,
            (_, true) => Self::Profiling,
        }
    }

    fn is_dev(&self) -> bool {
        matches!(self, Self::Dev)
    }

    fn is_not_dev(&self) -> bool {
        !self.is_dev()
    }

    fn env_name(&self) -> &str {
        match self {
            Self::Dev => "DEV",
            Self::Profiling => "PROFILING",
            Self::Release => "RELEASE",
        }
    }

    fn target_profile_folder(&self) -> &str {
        match self {
            Self::Dev => "debug",
            Self::Profiling => "profiling",
            Self::Release => "release",
        }
    }
}

#[throws]
#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    match args {
        Args::New { project_name, here } => command::new(project_name, here).await?,
        Args::Start {
            release,
            profiling,
            open,
        } => command::start(BuildMode::new(release, profiling), open).await?,
        Args::Build {
            release,
            profiling,
            frontend_dist,
            hosting,
        } => command::build(BuildMode::new(release, profiling), frontend_dist, hosting).await?,
    }
}
