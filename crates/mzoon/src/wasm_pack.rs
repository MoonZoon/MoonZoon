use std::process::Command;
use anyhow::{bail, Context, Result, anyhow};
use flate2::read::GzDecoder;
use tar::Archive;
use const_format::{concatcp, formatcp};
use std::path::PathBuf;
use crate::helper::download;

const VERSION: &str = "0.9.1";

// -- public --

pub fn check_or_install_wasm_pack() -> Result<()> {
    const DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/rustwasm/wasm-pack/releases/download/v{version}/wasm-pack-v{version}-{target}.tar.gz",
        version = VERSION,
        target = env!("TARGET"),
    );
    
    if check_wasm_pack().is_ok() { return Ok(()) }

    println!("Installing wasm-pack...");
    let tar_gz  = download(DOWNLOAD_URL)
        .context(formatcp!("Failed to download wasm-pack from the url '{}'", DOWNLOAD_URL))?;
    unpack_wasm_pack(tar_gz).context("Failed to unpack wasm-pack")?;
    Ok(println!("wasm-pack installed"))
}

pub fn build_with_wasm_pack(release: bool) -> Result<()> {
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
    Command::new("frontend/wasm-pack")
        .args(&args)
        .status()
        .context("Failed to get frontend build status")?
        .success()
        .then(||())
        .ok_or(anyhow!("Failed to build frontend"))    
}

// -- private --

fn check_wasm_pack() -> Result<()> {
    const EXPECTED_VERSION_OUTPUT: &str = concatcp!("wasm-pack ", VERSION, "\n");

    let version = Command::new("frontend/wasm-pack")
        .args(&["-V"])
        .output()?
        .stdout;

    if version == EXPECTED_VERSION_OUTPUT.as_bytes() {
        return Ok(())
    }
    bail!(concatcp!("wasm-pack's expected version is ", VERSION))
}

fn unpack_wasm_pack(tar_gz: Vec<u8>) -> Result<()> {
    let tar = GzDecoder::new(tar_gz.as_slice());
    let mut archive = Archive::new(tar);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path.file_stem().ok_or(anyhow!("Entry without a file name"))? == "wasm-pack" {
            let mut destination = PathBuf::from("frontend");
            destination.push(path.file_name().unwrap());
            entry.unpack(destination)?;
            return Ok(())
        }
    }
    bail!("Failed to find wasm-pack in the downloaded archive")
}
