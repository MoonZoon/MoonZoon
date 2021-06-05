use std::process::{Command, Child};
use anyhow::{Context, Error};
use fehler::throws;
use cargo_metadata::MetadataCommand;

#[throws]
pub fn run_backend(release: bool) -> Child {
    println!("Run backend");
 
    let mut target_directory = MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory;

    if release {
        target_directory.push("release")
    } else {
        target_directory.push("debug")
    };
    target_directory.push("backend");

    Command::new(target_directory).spawn().context("Failed to run backend")?
}
