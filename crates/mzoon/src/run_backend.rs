use tokio::process::{Command, Child};
use anyhow::{Context, Error};
use fehler::throws;
use cargo_metadata::MetadataCommand;
use apply::{Also, Apply};

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
