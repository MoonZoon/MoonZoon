use anyhow::Error;
use fehler::throws;
use std::path::PathBuf;
use tar::Archive;

static NEW_PROJECT_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/new_project.tar"));

#[throws]
pub async fn new(path: PathBuf) {
    let mut new_project_tar = Archive::new(NEW_PROJECT_TAR);
    new_project_tar.unpack(path)?;
    print!("New project created");
}
