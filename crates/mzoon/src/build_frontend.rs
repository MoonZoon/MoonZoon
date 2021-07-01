use crate::helper::{
    visit_files, AsyncReadToVec, BrotliFileCompressor, FileCompressor, GzipFileCompressor,
};
use crate::wasm_pack::{build_with_wasm_pack, check_or_install_wasm_pack};
use anyhow::{Context, Error};
use const_format::concatcp;
use fehler::throws;
use futures::TryStreamExt;
use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::try_join;
use uuid::Uuid;

// -- public --

#[throws]
pub async fn build_frontend(release: bool, cache_busting: bool) {
    println!("Building frontend...");
    check_or_install_wasm_pack().await?;
    remove_pkg().await?;
    build_with_wasm_pack(release).await?;

    let build_id = Uuid::new_v4().as_u128();
    let (wasm_file_path, js_file_path, _) = try_join!(
        rename_wasm_file(build_id, cache_busting),
        rename_js_file(build_id, cache_busting),
        write_build_id(build_id),
    )?;
    if release {
        compress_pkg(wasm_file_path.as_ref(), js_file_path.as_ref()).await?;
    }
    println!("Frontend built");
}

// -- private --

#[throws]
async fn remove_pkg() {
    let pkg_path = Path::new("frontend/pkg");
    if pkg_path.is_dir() {
        fs::remove_dir_all(pkg_path)
            .await
            .context("Failed to remove pkg")?;
    }
}

#[throws]
async fn write_build_id(build_id: u128) {
    fs::write("frontend/pkg/build_id", build_id.to_string())
        .await
        .context("Failed to write the frontend build id")?;
}

#[throws]
async fn rename_wasm_file(build_id: u128, cache_busting: bool) -> Cow<'static, str> {
    const PATH: &str = "frontend/pkg/frontend_bg";
    const EXTENSION: &str = ".wasm";
    const ORIGINAL_PATH: &str = concatcp!(PATH, EXTENSION);

    if !cache_busting {
        return Cow::from(ORIGINAL_PATH);
    };

    let new_path = format!("{}_{}{}", PATH, build_id, EXTENSION);
    fs::rename(ORIGINAL_PATH, &new_path)
        .await
        .context("Failed to rename the wasm file in the pkg directory")?;

    Cow::from(new_path)
}

#[throws]
async fn rename_js_file(build_id: u128, cache_busting: bool) -> Cow<'static, str> {
    const PATH: &str = "frontend/pkg/frontend";
    const EXTENSION: &str = ".js";
    const ORIGINAL_PATH: &str = concatcp!(PATH, EXTENSION);

    if !cache_busting {
        return Cow::from(ORIGINAL_PATH);
    };

    let new_path = format!("{}_{}{}", PATH, build_id, EXTENSION);
    fs::rename(ORIGINAL_PATH, &new_path)
        .await
        .context("Failed to rename the JS file in the pkg directory")?;

    Cow::from(new_path)
}

#[throws]
async fn compress_pkg(wasm_file_path: impl AsRef<Path>, js_file_path: impl AsRef<Path>) {
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
    .with_context(|| format!("Failed to create compressed files for {:?}", file_path))?
}
