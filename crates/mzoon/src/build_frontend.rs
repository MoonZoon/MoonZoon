use crate::helper::{
    visit_files, AsyncReadToVec, BrotliFileCompressor, FileCompressor, GzipFileCompressor,
};
use crate::wasm_bindgen::{build_with_wasm_bindgen, check_or_install_wasm_bindgen};
use crate::wasm_opt::{check_or_install_wasm_opt, optimize_with_wasm_opt};
use crate::BuildMode;
use anyhow::{anyhow, Context, Error};
use bool_ext::BoolExt;
use const_format::concatcp;
use fehler::throws;
use futures::TryStreamExt;
use std::{borrow::Cow, path::Path, sync::Arc};
use tokio::{fs, process::Command, try_join};
use uuid::Uuid;

// -- public --

#[throws]
pub async fn build_frontend(build_mode: BuildMode, cache_busting: bool, frontend_dist: bool) {
    println!("Building frontend...");
    compile_with_cargo(build_mode).await?;

    remove_pkg().await?;

    check_or_install_wasm_bindgen().await?;
    build_with_wasm_bindgen(build_mode).await?;

    if build_mode.is_not_dev() {
        check_or_install_wasm_opt().await?;
        optimize_with_wasm_opt(build_mode).await?;
    }

    let build_id = Uuid::new_v4().as_u128();
    let (wasm_file_path, js_file_path, _) = try_join!(
        rename_wasm_file(build_id, cache_busting),
        rename_js_file(build_id, cache_busting),
        write_build_id(build_id),
    )?;
    if build_mode.is_not_dev() && !frontend_dist {
        compress_pkg(wasm_file_path.as_ref(), js_file_path.as_ref()).await?;
    }
    println!("Frontend built");
}

// -- private --

#[throws]
pub async fn compile_with_cargo(build_mode: BuildMode) {
    let mut args = vec![
        "build",
        "--bin",
        "frontend",
        "--target",
        "wasm32-unknown-unknown",
    ];
    match build_mode {
        BuildMode::Dev => (),
        BuildMode::Profiling => args.extend(["--profile", "profiling"]),
        BuildMode::Release => args.push("--release"),
    }

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#configuration-environment-variables
    let mut cargo_configs = Vec::new();
    if build_mode.is_not_dev() {
        cargo_configs.extend([("OPT_LEVEL", "z"), ("CODEGEN_UNITS", "1"), ("LTO", "true")]);
    }
    if let BuildMode::Profiling = build_mode {
        cargo_configs.extend([("DEBUG", "true"), ("INHERITS", "release")]);
    }

    let profile_env_name = build_mode.env_name();
    let envs = cargo_configs
        .into_iter()
        .map(|(key, value)| (format!("CARGO_PROFILE_{profile_env_name}_{key}"), value));

    Command::new("cargo")
        .args(&args)
        .envs(envs)
        .status()
        .await
        .context("Failed to get frontend compilation status")?
        .success()
        .err(anyhow!("Failed to compile frontend"))?;
}

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
    static SNIPPETS_PATH: &str = "frontend/pkg/snippets";
    try_join!(
        create_compressed_files(wasm_file_path),
        create_compressed_files(js_file_path),
        async {
            if fs::metadata(SNIPPETS_PATH).await.is_ok() {
                visit_files(SNIPPETS_PATH)
                    .try_for_each_concurrent(None, |file| create_compressed_files(file.path()))
                    .await?;
            }
            Ok(())
        }
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
