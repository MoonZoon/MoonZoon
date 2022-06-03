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

const VERSION: &str = "0.2.80";

// -- public --

#[throws]
pub async fn check_or_install_wasm_bindgen() {
    if check_wasm_bindgen().await.is_ok() {
        return;
    }

    const TARGET: &str = env!("TARGET");
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            const NEAREST_TARGET: &str = "x86_64-apple-darwin";
        } else if #[cfg(target_os = "windows")] {
            const NEAREST_TARGET: &str = "x86_64-pc-windows-msvc";
        } else if #[cfg(target_os = "linux")] {
            const NEAREST_TARGET: &str = "x86_64-unknown-linux-musl";
        } else {
            compile_error!("wasm-bindgen pre-compiled binary hasn't been found for the target platform '{TARGET}'");
        }
    }
    const DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/rustwasm/wasm-bindgen/releases/download/{VERSION}/wasm-bindgen-{VERSION}-{NEAREST_TARGET}.tar.gz"
    );

    println!("Installing wasm-bindgen...");
    if TARGET != NEAREST_TARGET {
        println!(
            "Pre-compiled wasm-bindgen binary '{NEAREST_TARGET}' will be used for the target platform '{TARGET}'"
        );
    }
    download(DOWNLOAD_URL)
        .await
        .context(formatcp!(
            "Failed to download wasm-bindgen from the url '{DOWNLOAD_URL}'"
        ))?
        .apply(unpack_wasm_bindgen)
        .context("Failed to unpack wasm-bindgen")?;
    println!("wasm-bindgen installed");
}

// https://rustwasm.github.io/wasm-bindgen/reference/cli.html
// https://webassembly.org/roadmap/
#[throws]
pub async fn build_with_wasm_bindgen(build_mode: BuildMode) {
    let mut args = vec![
        "--target",
        "web",
        "--no-typescript",
        // @TODO/NOTE Fails in runtime even with `wasm-opt --enable-reference-types` (v.108). 
        // "--reference-types",
        "--weak-refs",
        "--out-dir",
        "frontend/pkg"
    ];
    if build_mode.is_dev() {
        args.push("--debug");
    }
    
    let target_profile_folder = match build_mode {
        BuildMode::Dev => "debug",
        BuildMode::Profiling => "profiling",
        BuildMode::Release => "release",
    };
    let wasm_path = format!("target/wasm32-unknown-unknown/{target_profile_folder}/frontend.wasm");
    args.push(&wasm_path);

    Command::new("frontend/wasm-bindgen")
        .args(&args)
        .status()
        .await
        .context("Failed to get frontend build status")?
        .success()
        .err(anyhow!("Failed to build frontend with wasm-bindgen"))?;
}

// -- private --

#[throws]
async fn check_wasm_bindgen() {
    const EXPECTED_VERSION_OUTPUT_START: &[u8] = concatcp!("wasm-bindgen ", VERSION).as_bytes();

    let version_output = Command::new("frontend/wasm-bindgen")
        .args(&["-V"])
        .output()
        .await?
        .stdout;

    if !version_output.starts_with(EXPECTED_VERSION_OUTPUT_START) {
        Err(anyhow!(concatcp!(
            "wasm-bindgen's expected version is ",
            VERSION
        )))?;
    }
}

#[throws]
fn unpack_wasm_bindgen(tar_gz: Vec<u8>) {
    let tar = GzDecoder::new(tar_gz.as_slice());
    let mut archive = Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let file_stem = path
            .file_stem()
            .ok_or(anyhow!("Entry without a file name"))?;
        if file_stem != "wasm-bindgen" {
            continue;
        }
        let mut destination = PathBuf::from("frontend");
        destination.push(path.file_name().unwrap());
        entry.unpack(destination)?;
        return;
    }
    Err(anyhow!(
        "Failed to find wasm-bindgen in the downloaded archive"
    ))?;
}
