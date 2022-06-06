use anyhow::Error;
use fehler::throws;
use std::path::PathBuf;

#[throws]
pub async fn new(path: PathBuf) {
    println!("NEW PATH: '{path:#?}'");
}
