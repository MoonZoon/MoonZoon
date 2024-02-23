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
    env, future,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use tokio::{fs, process::Command, select, sync::watch, try_join};
use uuid::Uuid;

// -- public --

#[throws]
pub async fn build_frontend(
    build_mode: BuildMode,
    cache_busting: bool,
    frontend_dist: bool,
    frontend_multithreading: bool,
    compilation_killer: Option<watch::Receiver<()>>,
) {
    println!("Building frontend...");

    let build_id = Uuid::new_v4().as_u128();
    env::set_var("FRONTEND_BUILD_ID", build_id.to_string());

    let web_workers = web_worker_workspace_members()?;

    compile_with_cargo(
        build_mode,
        "frontend",
        frontend_multithreading,
        compilation_killer.clone(),
    )
    .await?;
    for WorkspaceMember { name, .. } in &web_workers {
        compile_with_cargo(
            build_mode,
            name,
            frontend_multithreading,
            compilation_killer.clone(),
        )
        .await?;
    }

    remove_pkg(Path::new("frontend/pkg")).await?;
    for WorkspaceMember { path, .. } in &web_workers {
        remove_pkg(&path.join("pkg")).await?;
    }

    check_or_install_wasm_bindgen().await?;

    build_with_wasm_bindgen(
        build_mode,
        "frontend",
        Path::new("frontend"),
        if frontend_multithreading {
            "no-modules"
        } else {
            "web"
        },
    )
    .await?;
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
async fn compile_with_cargo(
    build_mode: BuildMode,
    bin_crate: &str,
    frontend_multithreading: bool,
    compilation_killer: Option<watch::Receiver<()>>,
) {
    // @TODO We have to run `rustup run <toolchain>` instead of `cargo +<toolchain>`
    // because `cargo +<toolchain>` is broken in Rustup on Windows.
    // https://github.com/rust-lang/rustup/issues/3036
    // Rewrite it back to `cargo +<toolchain>` once the issue is fixed.
    let mut args = vec!["run"];

    // @TODO `args: Vec<Cow<str>>`? or a macro/builder to build the `Vec`?
    #[allow(unused_assignments)]
    let mut active_toolchain = String::new();

    if frontend_multithreading {
        args.push("nightly")
    } else {
        // @TODO Rustup requires to select a toolchain but we don't want to force `stable`.
        // Also `default/active` or `""` is not a valid value (anymore?) - https://github.com/rust-lang/rustup/issues/3304.
        // So we need to find out the default/active toolchain before calling `rustup run`.
        let active_toolchain_stdout = Command::new("rustup")
            .args(["show", "active-toolchain"])
            .output()
            .await?
            .stdout;
        let active_toolchain_stdout = str::from_utf8(&active_toolchain_stdout)?;
        // `active_toolchain_stdout` examples:
        // - 'stable-x86_64-pc-windows-msvc (environment override by RUSTUP_TOOLCHAIN)'
        // - 'stable-x86_64-pc-windows-msvc (default)'
        active_toolchain = active_toolchain_stdout
            .split_once(' ')
            .map(|(toolchain, _)| toolchain)
            .unwrap_or(active_toolchain_stdout)
            .to_owned();
        args.push(&active_toolchain);
    }
    args.extend(vec![
        "cargo",
        "build",
        "--bin",
        &bin_crate,
        "--target",
        "wasm32-unknown-unknown",
    ]);
    if frontend_multithreading {
        args.extend(["--features", "zoon/frontend_multithreading"]);
    }
    match build_mode {
        BuildMode::Dev => (),
        BuildMode::Profiling => args.extend(["--profile", "profiling"]),
        BuildMode::Release => args.push("--release"),
    }
    if frontend_multithreading {
        // @TODO: It requires to run `rustup component add rust-src --toolchain nightly`
        // if the `rust-src` is not installed. Take it into account while implementing auto-installation.
        // Related MoonZoon issue: https://github.com/MoonZoon/MoonZoon/issues/115
        args.extend(["-Z", "build-std=panic_abort,std"]);
    }

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#configuration-environment-variables
    let mut cargo_configs = Vec::new();
    if build_mode.is_dev() {
        cargo_configs.push(("DEBUG", "false"));
    } else {
        cargo_configs.extend([("OPT_LEVEL", "z"), ("CODEGEN_UNITS", "1")]);
        if !frontend_multithreading {
            // @TODO / NOTE: LTO breaks the app when it's building for MT support
            cargo_configs.push(("LTO", "true"))
        }
    }
    if let BuildMode::Profiling = build_mode {
        cargo_configs.extend([("DEBUG", "true"), ("INHERITS", "release")]);
    }

    let mut envs: Vec<(String, &str)> = vec![];

    #[allow(unused_assignments)]
    let mut rustflags_value = String::new();
    if frontend_multithreading {
        let mut rustflags = vec![
            "-C target-feature=+atomics,+bulk-memory,+mutable-globals",
            // @TODO is possible to disable warnings like `warning: unstable feature specified for `-Ctarget-feature`: `atomics`?
            // https://github.com/rust-lang/rust/blob/2dbd6233ccdb2cd4b621a5e839a95c3fbbc0c375/compiler/rustc_codegen_llvm/src/llvm_util.rs#L570
            // Perhaps it won't show again once we can use `cargo +nightly` on Windows
        ];
        if build_mode.is_not_dev() {
            rustflags.push("-Z location-detail=none");
        }
        rustflags_value = rustflags.join(" ");
        envs.push(("RUSTFLAGS".to_owned(), &rustflags_value));
    }

    let profile_env_name = build_mode.env_name();
    let envs = cargo_configs
        .into_iter()
        .map(|(key, value)| {
            (
                format!("CARGO_PROFILE_{profile_env_name}_{key}").into(),
                value,
            )
        })
        .chain(envs);

    let mut process = Command::new("rustup")
        .args(&args)
        .envs(envs)
        .spawn()
        .context("Failed to start {bin_crate} compilation")?;

    let compilation_killer_or_pending = async move {
        if let Some(mut compilation_killer) = compilation_killer {
            let _ = compilation_killer.changed().await;
            println!("Stop compilation");
        } else {
            future::pending::<()>().await;
        }
    };
    select! {
        result = process.wait() => {
            result
                .context("Failed to get {bin_crate} compilation status")?
                .success()
                .err(anyhow!("Failed to compile {bin_crate}"))?;
        }
        _ = compilation_killer_or_pending => {
            process
                .kill()
                .await
                .context("Failed to kill {bin_crate} compilation")?;
            Err(anyhow!("Compilation stopped"))?;
        }
    };
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
