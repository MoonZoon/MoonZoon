use std::process::{Command, Stdio};
use anyhow::{bail, Context, Result, anyhow};
use flate2::read::GzDecoder;
use tar::Archive;
use const_format::formatcp;
use std::path::PathBuf;

// -- public --

pub fn check_or_install_wasm_pack() -> Result<()> {
    const WASM_PACK_VERSION: &str = "0.9.1";
    const WASM_PACK_DOWNLOAD_URL: &str = formatcp!(
        "https://github.com/rustwasm/wasm-pack/releases/download/v{version}/wasm-pack-v{version}-{target}.tar.gz",
        version = WASM_PACK_VERSION,
        target = env!("TARGET"),
    );
    
    if check_wasm_pack(WASM_PACK_VERSION) { return Ok(()) }

    println!("Installing wasm-pack...");
    let tar_gz  = download(WASM_PACK_DOWNLOAD_URL)
        .context(formatcp!("Failed to download wasm-pack from the url '{}'", WASM_PACK_DOWNLOAD_URL))?;
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

fn check_wasm_pack(version: &str) -> bool {
    let status = Command::new("frontend/wasm-pack")
        .args(&["-V"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => true,
        _ => false,
    }
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

fn download(url: impl AsRef<str>) -> Result<Vec<u8>> {
    let bytes = attohttpc::get(url)
        .send()?
        .error_for_status()?
        .bytes()?;
    Ok(bytes)
}
