use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::{try_join, join, spawn};
use tokio::task::JoinHandle;
use tokio::time::Duration;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use uuid::Uuid;
use anyhow::{bail, Context, Result};
use std::sync::Arc;
use futures::TryStreamExt;
use crate::config::Config;
use crate::file_compressor::{BrotliFileCompressor, GzipFileCompressor, FileCompressor};
use crate::visit_files::visit_files;
use crate::project_watcher::ProjectWatcher;

pub fn check_wasm_pack() -> Result<()> {
    let status = Command::new("wasm-pack")
        .args(&["-V"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => Ok(()),
        _ => bail!("Cannot find `wasm-pack`! Please install it by `cargo install wasm-pack` or download/install pre-built binaries into a globally available directory."),
    }
}

pub fn start_frontend_watcher(config: &Config, release: bool, debounce_time: Duration) -> JoinHandle<Result<()>> {
    let reload_url = Arc::new(format!(
        "{protocol}://localhost:{port}/api/reload",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    ));
    let cache_busting = config.cache_busting;
    let paths = config.watch.frontend.clone();

    spawn(async move {
        let project_watcher = ProjectWatcher::new(paths, debounce_time);
        let mut debounced_receiver = project_watcher.start().await?;

        let mut build_task = None::<JoinHandle<()>>;
        while debounced_receiver.recv().await.is_some() {
            println!("Build frontend");
            if let Some(build_task) = build_task.take() {
                build_task.abort();
            }
            let reload_url = Arc::clone(&reload_url);
            build_task = Some(spawn(async move {
                match build_frontend(release, cache_busting).await {
                    Ok(()) => {
                        println!("Reload frontend");
                        let response = attohttpc::post(reload_url.as_str())
                            .danger_accept_invalid_certs(true)
                            .send();
                        if let Err(error) = response {
                            eprintln!("Failed to send the frontend reload request: {:#?}", error);
                        }
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }));
        }
        
        Ok(())
    })
}

pub async fn build_frontend(release: bool, cache_busting: bool) -> Result<()> {
    println!("Building frontend...");

    let old_build_id = fs::read_to_string("frontend/pkg/build_id")
        .await
        .ok()
        .map(|uuid| uuid.parse::<u128>().map(|uuid| uuid).unwrap_or_default());

    if let Some(old_build_id) = old_build_id {
        let old_wasm = format!("frontend/pkg/frontend_bg_{}.wasm", old_build_id);
        let old_js = format!("frontend/pkg/frontend_{}.js", old_build_id);
        let _ = join!(
            fs::remove_file(&old_wasm),
            fs::remove_file(&old_js),
            fs::remove_file(format!("{}.br", &old_wasm)),
            fs::remove_file(format!("{}.br", &old_js)),
            fs::remove_file(format!("{}.gz", &old_wasm)),
            fs::remove_file(format!("{}.gz", &old_js)),
            fs::remove_dir_all("frontend/pkg/snippets"),
        );
    }

    let mut args = vec![
        "--log-level",
        "warn",
        "build",
        "frontend",
        "--target",
        "web",
        "--no-typescript",
    ];
    if !release {
        args.push("--dev");
    }
    let success = Command::new("wasm-pack")
        .args(&args)
        .status()
        .context("Failed to get frontend build status")?
        .success();
    if success {
        let build_id = cache_busting
            .then(|| Uuid::new_v4().as_u128())
            .unwrap_or_default();

        let wasm_file_path = Path::new("frontend/pkg/frontend_bg.wasm");
        let new_wasm_file_path =
            PathBuf::from(format!("frontend/pkg/frontend_bg_{}.wasm", build_id));
        let js_file_path = Path::new("frontend/pkg/frontend.js");
        let new_js_file_path = PathBuf::from(format!("frontend/pkg/frontend_{}.js", build_id));

        try_join!(
            async { fs::rename(wasm_file_path, &new_wasm_file_path).await.context("Failed to rename the Wasm file in the pkg directory") },
            async { fs::rename(js_file_path, &new_js_file_path).await.context("Failed to rename the JS file in the pkg directory") },
            async { fs::write("frontend/pkg/build_id", build_id.to_string()).await.context("Failed to write the frontend build id") },
        )?;

        if release {
            compress_pkg(&new_wasm_file_path, &new_js_file_path).await?;
        }
        return Ok(println!("Frontend built"))
    }
    bail!("Failed to build frontend")
}

pub async fn compress_pkg(wasm_file_path: &Path, js_file_path: &Path) -> Result<()> {
    create_compressed_files(wasm_file_path).await?;
    create_compressed_files(js_file_path).await?;

    visit_files("frontend/pkg/snippets")
        .try_for_each_concurrent(None, |file| create_compressed_files(file.path()))
        .await
}

pub async fn create_compressed_files(file_path: impl AsRef<Path>) -> Result<()> {
    let mut content = Vec::new();
    fs::File::open(&file_path).await?.read_to_end(&mut content).await?;
    let content = Arc::new(content);

    try_join!(
        async { BrotliFileCompressor::compress_file(Arc::clone(&content), file_path.as_ref(), "br").await? }, 
        async { GzipFileCompressor::compress_file(Arc::clone(&content), file_path.as_ref(), "gz").await? },
    ).context("Failed to create compressed files")?;
    Ok(())
}
