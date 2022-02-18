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

const VERSION: &str = "0.10.2";

// -- public --

#[throws]
pub async fn check_or_install_wasm_pack() {
    if check_wasm_pack().await.is_ok() {
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
            compile_error!("wasm-pack pre-compiled binary hasn't been found for the target platform '{}'", TARGET);
        }
    }
    const DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/rustwasm/wasm-pack/releases/download/v{version}/wasm-pack-v{version}-{target}.tar.gz",
        version = VERSION,
        target = NEAREST_TARGET,
    );

    println!("Installing wasm-pack...");
    if TARGET != NEAREST_TARGET {
        println!(
            "Pre-compiled wasm-pack binary '{}' will be used for the target platform '{}'",
            NEAREST_TARGET, TARGET
        );
    }
    download(DOWNLOAD_URL)
        .await
        .context(formatcp!(
            "Failed to download wasm-pack from the url '{}'",
            DOWNLOAD_URL
        ))?
        .apply(unpack_wasm_pack)
        .context("Failed to unpack wasm-pack")?;
    println!("wasm-pack installed");
}

#[throws]
pub async fn build_with_wasm_pack(build_mode: BuildMode) {
    let mut args = vec![
        "--log-level",
        "warn",
        "build",
        "frontend",
        "--target",
        "web",
        "--no-typescript",
    ];
    // https://rustwasm.github.io/docs/wasm-pack/commands/build.html#profile
    match build_mode {
        BuildMode::Dev => args.push("--dev"),
        // @TODO does it work? See
        // https://github.com/rustwasm/wasm-pack/blob/4ae6306570a0011246c39c8028a4f11a4236f54b/src/build/mod.rs#L92-L99
        // and https://github.com/rustwasm/wasm-pack/issues/797
        BuildMode::Profiling => args.push("--profiling"),
        BuildMode::Release => (),
    }
    Command::new("frontend/wasm-pack")
        .args(&args)
        .status()
        .await
        .context("Failed to get frontend build status")?
        .success()
        .err(anyhow!("Failed to build frontend"))?;
}

// -- private --

#[throws]
async fn check_wasm_pack() {
    const EXPECTED_VERSION_OUTPUT: &[u8] = concatcp!("wasm-pack ", VERSION, "\n").as_bytes();

    let version = Command::new("frontend/wasm-pack")
        .args(&["-V"])
        .output()
        .await?
        .stdout;

    if version != EXPECTED_VERSION_OUTPUT {
        Err(anyhow!(concatcp!(
            "wasm-pack's expected version is ",
            VERSION
        )))?;
    }
}

#[throws]
fn unpack_wasm_pack(tar_gz: Vec<u8>) {
    let tar = GzDecoder::new(tar_gz.as_slice());
    let mut archive = Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let file_stem = path
            .file_stem()
            .ok_or(anyhow!("Entry without a file name"))?;
        if file_stem != "wasm-pack" {
            continue;
        }
        let mut destination = PathBuf::from("frontend");
        destination.push(path.file_name().unwrap());
        entry.unpack(destination)?;
        return;
    }
    Err(anyhow!(
        "Failed to find wasm-pack in the downloaded archive"
    ))?;
}
