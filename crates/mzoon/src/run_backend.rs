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
        .also(|directory| {
            directory.push(if build_mode.is_dev() {
                "debug"
            } else {
                "release"
            })
        })
        .also(|directory| directory.push("backend"))
        .apply(Command::new)
        .spawn()
        .context("Failed to run backend")?
}
