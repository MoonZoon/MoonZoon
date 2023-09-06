use crate::config::Config;
use crate::helper::{
    download, localhost_url,
    workspace_member::{web_worker_workspace_members, WorkspaceMember},
};
use crate::run_backend::run_backend;
use crate::{BuildMode, Hosting};
use anyhow::Error;
use const_format::concatcp;
use fehler::throws;
use fs_extra::dir;
use std::path::Path;
use tokio::{fs, task};

const FRONTEND_DIST_DIR: &str = "frontend_dist";
const API_DIR: &str = concatcp!(FRONTEND_DIST_DIR, "/_api");

// -- public --

#[throws]
pub async fn create_frontend_dist(
    build_mode: BuildMode,
    config: &Config,
    hosting: Option<Hosting>,
) {
    println!("Creating frontend_dist...");

    recreate_api_dir_with_frontend_dist().await?;
    recreate_index_html(build_mode, config).await?;
    task::spawn_blocking(copy_pkg_public_sync).await??;
    task::spawn_blocking(copy_web_workers_sync).await??;
    if let Some(hosting) = hosting {
        create_hosting_files(hosting).await?;
    }

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

#[throws]
fn copy_web_workers_sync() {
    let workspace_members = web_worker_workspace_members()?;
    if workspace_members.is_empty() {
        return;
    }

    let web_workers_path = Path::new(API_DIR).join("web_workers");
    dir::create(&web_workers_path, false)?;

    let copy_options = dir::CopyOptions::new();
    for WorkspaceMember { name, mut path, .. } in workspace_members {
        let web_worker_path = web_workers_path.join(&name);
        dir::create(&web_worker_path, false)?;

        path.push("pkg");
        dir::copy(path, web_worker_path, &copy_options)?;
    }
}

#[throws]
async fn create_hosting_files(hosting: Hosting) {
    match hosting {
        Hosting::Netlify => {
            const NETLIFY_ROUTER: &str = concatcp!(FRONTEND_DIST_DIR, "/netlify.toml");
            if fs::metadata(NETLIFY_ROUTER).await.is_err() {
                fs::write(
                    NETLIFY_ROUTER,
                    include_str!("../hosting_configs/netlify.toml"),
                )
                .await?;
                println!("netlify.toml added to frontend_dist");
            }
        }
    }
}
