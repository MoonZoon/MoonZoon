use tokio::fs;
use tokio::{try_join, join};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use anyhow::{Context, Error};
use std::sync::Arc;
use futures::TryStreamExt;
use fehler::throws;
use crate::wasm_pack::{check_or_install_wasm_pack, build_with_wasm_pack};
use crate::helper::{BrotliFileCompressor, GzipFileCompressor, FileCompressor, visit_files, AsyncReadToVec};

// -- public --

#[throws]
pub async fn build_frontend(release: bool, cache_busting: bool) {
    println!("Building frontend...");

    check_or_install_wasm_pack()?;

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

    build_with_wasm_pack(release)?;

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
    println!("Frontend built");
}

// -- private --

#[throws]
async fn compress_pkg(wasm_file_path: &Path, js_file_path: &Path) {
    try_join!(
        create_compressed_files(wasm_file_path),
        create_compressed_files(js_file_path),
        visit_files("frontend/pkg/snippets")
            .try_for_each_concurrent(None, |file| create_compressed_files(file.path()))
    )?
}

#[throws]
async fn create_compressed_files(file_path: impl AsRef<Path>) {
    let file_path = file_path.as_ref();
    let content = Arc::new(fs::File::open(&file_path).await?.read_to_vec().await?);

    try_join!(
        BrotliFileCompressor::compress_file(Arc::clone(&content), file_path, "br"), 
        GzipFileCompressor::compress_file(content, file_path, "gz"),
    )
    .with_context(|| format!("Failed to create compressed files for {:#?}", file_path))?
}
