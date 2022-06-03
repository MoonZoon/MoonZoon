use crate::{helper::download, BuildMode};
use anyhow::{anyhow, Context, Error};
use apply::Apply;
use bool_ext::BoolExt;
use cfg_if::cfg_if;
use const_format::{concatcp, formatcp};
use fehler::throws;
use flate2::read::GzDecoder;
use std::path::PathBuf;
use tar::Archive;
use tokio::process::Command;

const VERSION: &str = "108";

// -- public --

#[throws]
pub async fn check_or_install_wasm_opt() {
    if check_wasm_opt().await.is_ok() {
        return;
    }

    const TARGET: &str = env!("TARGET");
    cfg_if! {
        if #[cfg(all(target_os = "macos", target_arch = "arm"))] {
            const ARCHIVE_PLATFORM: &str = "arm64-macos";
        } else if #[cfg(target_os = "macos")] {
            const ARCHIVE_PLATFORM: &str = "x86_64-macos";
        } else if #[cfg(target_os = "windows")] {
            const ARCHIVE_PLATFORM: &str = "x86_64-windows";
        } else if #[cfg(target_os = "linux")] {
            const ARCHIVE_PLATFORM: &str = "x86_64-linux";
        } else {
            compile_error!("wasm-opt pre-compiled binary hasn't been found for the target platform '{TARGET}'");
        }
    }
    const DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/WebAssembly/binaryen/releases/download/version_{VERSION}/binaryen-version_{VERSION}-{ARCHIVE_PLATFORM}.tar.gz",
    );

    println!("Installing wasm-opt...");
    println!(
        "Pre-compiled wasm-opt binary '{ARCHIVE_PLATFORM}' will be used for the target platform '{TARGET}'"
    );

    download(DOWNLOAD_URL)
        .await
        .context(formatcp!(
            "Failed to download wasm-opt from the url '{DOWNLOAD_URL}'"
        ))?
        .apply(unpack_wasm_opt)
        .context("Failed to unpack wasm-opt")?;
    println!("wasm-opt installed");
}

#[throws]
pub async fn optimize_with_wasm_opt(build_mode: BuildMode) {
    let mut args = vec![
        "frontend/pkg/frontend_bg.wasm",
        "--output",
        "frontend/pkg/frontend_bg.wasm",
    ];
    if build_mode.is_not_dev() {
        args.push("-Oz");
    }
    if let BuildMode::Profiling = build_mode {
        args.push("--debuginfo");
    }
    Command::new("frontend/wasm-opt")
        .args(&args)
        .status()
        .await
        .context("Failed to get frontend optimization status")?
        .success()
        .err(anyhow!("Failed to optimize frontend with wasm-opt"))?;
}

// -- private --

#[throws]
async fn check_wasm_opt() {
    const EXPECTED_VERSION_OUTPUT_START: &[u8] = concatcp!("wasm-opt version ", VERSION).as_bytes();

    let version_output = Command::new("frontend/wasm-opt")
        .args(&["--version"])
        .output()
        .await?
        .stdout;

    if !version_output.starts_with(EXPECTED_VERSION_OUTPUT_START) {
        Err(anyhow!(concatcp!(
            "wasm-opt's expected version is ",
            VERSION
        )))?;
    }
}

#[throws]
fn unpack_wasm_opt(tar_gz: Vec<u8>) {
    let tar = GzDecoder::new(tar_gz.as_slice());
    let mut archive = Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let file_stem = path
            .file_stem()
            .ok_or(anyhow!("Entry without a file name"))?;
        if file_stem != "wasm-opt" {
            continue;
        }
        let mut destination = PathBuf::from("frontend");
        destination.push(path.file_name().unwrap());
        entry.unpack(destination)?;
        return;
    }
    Err(anyhow!(
        "Failed to find wasm-opt in the downloaded archive"
    ))?;
}
