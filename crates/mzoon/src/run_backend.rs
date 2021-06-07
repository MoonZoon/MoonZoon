use anyhow::{Context, Error};
use apply::{Also, Apply};
use cargo_metadata::MetadataCommand;
use fehler::throws;
use tokio::process::{Child, Command};

#[throws]
pub fn run_backend(release: bool) -> Child {
    println!("Run backend");
    MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory
        .also(|directory| directory.push(if release { "release" } else { "debug" }))
        .also(|directory| directory.push("backend"))
        .apply(Command::new)
        .spawn()
        .context("Failed to run backend")?
}
