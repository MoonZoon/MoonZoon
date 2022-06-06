use crate::config::Config;
use crate::helper::{download, localhost_url};
use crate::run_backend::run_backend;
use crate::BuildMode;
use anyhow::Error;
use const_format::concatcp;
use fehler::throws;
use fs_extra::dir;
use tokio::{fs, task};

const FRONTEND_DIST_DIR: &str = "frontend_dist";
const API_DIR: &str = concatcp!(FRONTEND_DIST_DIR, "/_api");

// -- public --

#[throws]
pub async fn create_frontend_dist(build_mode: BuildMode, config: &Config) {
    println!("Creating frontend_dist...");

    recreate_api_dir_with_frontend_dist().await?;
    recreate_index_html(build_mode, config).await?;
    task::spawn_blocking(copy_pkg_public_sync).await??;

    println!("frontend_dist created");
}

// -- private --

#[throws]
async fn recreate_api_dir_with_frontend_dist() {
    if !fs::metadata(API_DIR).await.is_err() {
        fs::remove_dir_all(API_DIR).await?;
    }
    fs::create_dir_all(API_DIR).await?;
}

#[throws]
async fn recreate_index_html(build_mode: BuildMode, config: &Config) {
    let server = run_backend(build_mode)?;
    let html = download(localhost_url(config)).await?;
    drop(server);

    fs::write(concatcp!(FRONTEND_DIST_DIR, "/index.html"), html).await?;
}

#[throws]
fn copy_pkg_public_sync() {
    let copy_options = dir::CopyOptions::new();

    dir::copy("frontend/pkg", API_DIR, &copy_options)?;
    dir::copy("public", API_DIR, &copy_options)?;
}
