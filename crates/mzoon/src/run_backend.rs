use crate::BuildMode;
use anyhow::{Context, Error};
use apply::{Also, Apply};
use cargo_metadata::MetadataCommand;
use fehler::throws;
use tokio::process::{Child, Command};

#[throws]
pub fn run_backend(build_mode: BuildMode) -> Child {
    println!("Run backend");
    MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory
        .also(|directory| directory.push(build_mode.target_profile_folder()))
        .also(|directory| directory.push("backend"))
        .apply(Command::new)
        .kill_on_drop(true)
        .spawn()
        .context("Failed to run backend")?
}
