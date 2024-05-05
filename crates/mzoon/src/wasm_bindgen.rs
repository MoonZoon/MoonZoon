use crate::{helper::download, BuildMode};
use anyhow::{anyhow, Context, Error};
use apply::Apply;
use bool_ext::BoolExt;
use cargo_metadata::MetadataCommand;
use cfg_if::cfg_if;
use const_format::{concatcp, formatcp};
use fehler::throws;
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR as SEP};
use tar::Archive;
use tokio::process::Command;

// NOTE: Sync with zoon's wasm-bindgen version.
const VERSION: &str = "0.2.92";

// -- public --

#[throws]
pub async fn check_or_install_wasm_bindgen() {
    if check_wasm_bindgen().await.is_ok() {
        return;
    }

    const TARGET: &str = env!("TARGET");
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            cfg_if! {
                if #[cfg(target_arch = "aarch64")] {
                    const NEAREST_TARGET: &str = "aarch64-apple-darwin";
                } else {
                    const NEAREST_TARGET: &str = "x86_64-apple-darwin";
                }
            }
        } else if #[cfg(target_os = "windows")] {
            const NEAREST_TARGET: &str = "x86_64-pc-windows-msvc";
        } else if #[cfg(target_os = "linux")] {
            cfg_if! {
                if #[cfg(target_arch = "aarch64")] {
                    const NEAREST_TARGET: &str = "aarch64-unknown-linux-gnu";
                } else {
                    const NEAREST_TARGET: &str = "x86_64-unknown-linux-musl";
                }
            }
        } else {
            compile_error!("wasm-bindgen pre-compiled binary hasn't been found for the target platform '{TARGET}'");
        }
    }
    const DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/rustwasm/wasm-bindgen/releases/download/{VERSION}/wasm-bindgen-{VERSION}-{NEAREST_TARGET}.tar.gz"
    );

    println!("Downloading & Installing wasm-bindgen {VERSION} ...");
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
pub async fn build_with_wasm_bindgen(
    build_mode: BuildMode,
    crate_name: &str,
    crate_path: &Path,
    target: &str,
) {
    let pkg_path = crate_path.join("pkg");
    let mut args: Vec<&OsStr> = vec![
        "--target".as_ref(),
        target.as_ref(),
        "--no-typescript".as_ref(),
        // @TODO uncomment when Safari 15.6.1 (August 18, 2022) is no longer needed
        // (including testing in a virtual machine)
        // "--reference-types".as_ref(),
        "--weak-refs".as_ref(),
        "--out-dir".as_ref(),
        pkg_path.as_os_str(),
    ];
    if build_mode.is_dev() {
        args.push("--debug".as_ref());
    }

    let target_path = MetadataCommand::new().no_deps().exec()?.target_directory;
    let target_profile_folder = build_mode.target_profile_folder();
    let wasm_path =
        format!("{target_path}{SEP}wasm32-unknown-unknown{SEP}{target_profile_folder}{SEP}{crate_name}.wasm");
    args.push(wasm_path.as_ref());

    Command::new("frontend/wasm-bindgen")
        .args(&args)
        .status()
        .await
        .context("Failed to get {crate_name} build status")?
        .success()
        .err(anyhow!("Failed to build {crate_name} with wasm-bindgen"))?;
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
