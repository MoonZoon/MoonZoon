use crate::helper::{
    visit_files,
    workspace_member::{web_worker_workspace_members, WorkspaceMember},
    AsyncReadToVec, BrotliFileCompressor, FileCompressor, GzipFileCompressor,
};
use crate::wasm_bindgen::{build_with_wasm_bindgen, check_or_install_wasm_bindgen};
use crate::wasm_opt::{check_or_install_wasm_opt, optimize_with_wasm_opt};
use crate::BuildMode;
use anyhow::{anyhow, Context, Error};
use bool_ext::BoolExt;
use fehler::throws;
use futures::TryStreamExt;
use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, process::Command, try_join};
use uuid::Uuid;

// -- public --

#[throws]
pub async fn build_frontend(build_mode: BuildMode, cache_busting: bool, frontend_dist: bool) {
    println!("Building frontend...");

    let build_id = Uuid::new_v4().as_u128();
    env::set_var("FRONTEND_BUILD_ID", build_id.to_string());

    let web_workers = web_worker_workspace_members()?;

    compile_with_cargo(build_mode, "frontend").await?;
    for WorkspaceMember { name, .. } in &web_workers {
        compile_with_cargo(build_mode, name).await?;
    }

    remove_pkg(Path::new("frontend/pkg")).await?;
    for WorkspaceMember { path, .. } in &web_workers {
        remove_pkg(&path.join("pkg")).await?;
    }

    check_or_install_wasm_bindgen().await?;

    build_with_wasm_bindgen(build_mode, "frontend", Path::new("frontend"), "web").await?;
    for WorkspaceMember { name, path, .. } in &web_workers {
        build_with_wasm_bindgen(build_mode, name, path, "no-modules").await?;
    }

    write_build_id(build_id).await?;

    if build_mode.is_not_dev() {
        check_or_install_wasm_opt().await?;
        optimize_with_wasm_opt(build_mode, "frontend", Path::new("frontend")).await?;
        for WorkspaceMember { name, path, .. } in &web_workers {
            optimize_with_wasm_opt(build_mode, name, path).await?;
        }
    }

    rename_and_compress_pkg_files(
        build_id,
        build_mode,
        cache_busting,
        frontend_dist,
        "frontend",
        Path::new("frontend"),
    )
    .await?;
    for WorkspaceMember { name, path, .. } in &web_workers {
        rename_and_compress_pkg_files(
            build_id,
            build_mode,
            cache_busting,
            frontend_dist,
            name,
            path,
        )
        .await?;
    }

    println!("Frontend built");
}

// -- private --

#[throws]
async fn rename_and_compress_pkg_files(
    build_id: u128,
    build_mode: BuildMode,
    cache_busting: bool,
    frontend_dist: bool,
    crate_name: &str,
    crate_path: &Path,
) {
    let (wasm_file_path, js_file_path, snippets_path) = try_join!(
        rename_wasm_file(build_id, cache_busting, crate_name, crate_path),
        rename_js_file(build_id, cache_busting, crate_name, crate_path),
        rename_snippets_folder(build_id, cache_busting, crate_name, crate_path),
    )?;

    update_snippet_paths_in_js_file(build_id, cache_busting, &js_file_path).await?;

    if build_mode.is_not_dev() && !frontend_dist {
        compress_pkg(wasm_file_path, js_file_path, snippets_path).await?;
    }
}

#[throws]
async fn compile_with_cargo(build_mode: BuildMode, bin_crate: &str) {
    let mut args = vec![
        "build",
        "--bin",
        &bin_crate,
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
    if build_mode.is_dev() {
        cargo_configs.push(("DEBUG", "false"));
    } else {
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
        .context("Failed to get {bin_crate} compilation status")?
        .success()
        .err(anyhow!("Failed to compile {bin_crate}"))?;
}

#[throws]
async fn remove_pkg(pkg_path: &Path) {
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
async fn rename_wasm_file(
    build_id: u128,
    cache_busting: bool,
    crate_name: &str,
    crate_path: &Path,
) -> PathBuf {
    let pkg_path = crate_path.join("pkg");
    let original_path = pkg_path.join(format!("{crate_name}_bg.wasm"));

    if !cache_busting {
        return original_path;
    };

    let new_path = pkg_path.join(format!("{crate_name}_bg_{build_id}.wasm"));

    fs::rename(original_path, &new_path).await.context(format!(
        "Failed to rename the wasm file in the pkg directory of the crate '{crate_name}'"
    ))?;
    new_path
}

#[throws]
async fn rename_snippets_folder(
    build_id: u128,
    cache_busting: bool,
    crate_name: &str,
    crate_path: &Path,
) -> PathBuf {
    let original_path = crate_path.join("pkg/snippets");

    if !cache_busting || fs::metadata(&original_path).await.is_err() {
        return original_path;
    };

    let new_path = crate_path.join(format!("pkg/snippets_{build_id}"));

    fs::rename(original_path, &new_path).await.context(format!(
        "Failed to rename the snippets folder in the pkg directory of the crate '{crate_name}'"
    ))?;

    new_path
}

#[throws]
async fn update_snippet_paths_in_js_file(
    build_id: u128,
    cache_busting: bool,
    js_file_path: impl AsRef<Path>,
) {
    if !cache_busting {
        return;
    };

    let js = fs::read_to_string(&js_file_path).await?.replace(
        "from './snippets/",
        &format!("from './snippets_{build_id}/"),
    );

    fs::write(js_file_path, js).await?;
}

#[throws]
async fn rename_js_file(
    build_id: u128,
    cache_busting: bool,
    crate_name: &str,
    crate_path: &Path,
) -> PathBuf {
    let pkg_path = crate_path.join("pkg");
    let original_path = pkg_path.join(format!("{crate_name}.js"));

    if !cache_busting {
        return original_path;
    };

    let new_path = pkg_path.join(format!("{crate_name}_{build_id}.js"));

    fs::rename(original_path, &new_path)
        .await
        .context("Failed to rename the JS file in the pkg directory of the crate '{crate_name}'")?;
    new_path
}

#[throws]
async fn compress_pkg(
    wasm_file_path: impl AsRef<Path>,
    js_file_path: impl AsRef<Path>,
    snippets_file_path: impl AsRef<Path>,
) {
    try_join!(
        create_compressed_files(wasm_file_path),
        create_compressed_files(js_file_path),
        async {
            if fs::metadata(snippets_file_path.as_ref()).await.is_ok() {
                visit_files(snippets_file_path.as_ref())
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
    .with_context(|| format!("Failed to create compressed files for {file_path:?}"))?
}
