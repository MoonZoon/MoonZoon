use brotli::{enc::backward_references::BrotliEncoderParams, BrotliCompress};
use flate2::bufread::GzEncoder;
use flate2::Compression;
use notify::{RecursiveMode, immediate_watcher, Watcher};
use tokio::fs::{self, DirEntry, File};
use tokio::{try_join, join, spawn};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use tokio::sync::mpsc;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use uuid::Uuid;
use anyhow::{bail, Context, Result};
use crate::config::Config;

pub fn check_wasm_pack() -> Result<()> {
    let status = Command::new("wasm-pack")
        .args(&["-V"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => Ok(()),
        _ => bail!("Cannot find `wasm-pack`! Please install it by `cargo install wasm-pack` or download/install pre-built binaries into a globally available directory."),
    }
}

pub fn start_frontend_watcher(config: &Config, release: bool) -> JoinHandle<Result<()>> {
    let reload_url = format!(
        "{protocol}://localhost:{port}/api/reload",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    );
    let cache_busting = config.cache_busting;
    let paths = config.watch.frontend;

    spawn(async move {
        let (sender, receiver) = mpsc::unbounded_channel();

        let watcher = immediate_watcher(|event| {
            if let Err(error) = event {
                return println!("Watch error: {:#?}", error);
            }
            sender.send(());
        }).context("Failed to create the frontend watcher")?;

        let configure_context = "Failed to configure the frontend watcher";
        watcher.configure(notify::Config::PreciseEvents(false)).context(configure_context)?;
        watcher.configure(notify::Config::NoticeEvents(false)).context(configure_context)?;
        watcher.configure(notify::Config::OngoingEvents(None)).context(configure_context)?;

        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }

        let (debounced_sender, debounced_receiver) = mpsc::unbounded_channel();

        spawn(async move {
            let task = None::<JoinHandle<()>>;
            while receiver.recv().await.is_some() {
                if let Some(task) = task {
                    task.abort();
                }
                task = Some(spawn(async move {
                    sleep(Duration::from_millis(100)).await; 
                    debounced_sender.send(());
                }));
            }
        });

        while debounced_receiver.recv().await.is_some() {
            println!("Build frontend");
            if build_frontend(release, cache_busting).await.is_ok() {
                println!("Reload frontend");
                attohttpc::post(&reload_url)
                    .danger_accept_invalid_certs(true)
                    .send()
                    .context("Failed to send the frontend reload request")?;
            }
        }
        Ok(())
    })
}

pub async fn build_frontend(release: bool, cache_busting: bool) -> Result<()> {
    let old_build_id = fs::read_to_string("frontend/pkg/build_id")
        .await
        .ok()
        .map(|uuid| uuid.parse::<u128>().map(|uuid| uuid).unwrap_or_default());

    if let Some(old_build_id) = old_build_id {
        let old_wasm = format!("frontend/pkg/frontend_bg_{}.wasm", old_build_id);
        let old_js = format!("frontend/pkg/frontend_{}.js", old_build_id);
        join!(
            fs::remove_file(&old_wasm),
            fs::remove_file(&old_js),
            fs::remove_file(format!("{}.br", &old_wasm)),
            fs::remove_file(format!("{}.br", &old_js)),
            fs::remove_file(format!("{}.gz", &old_wasm)),
            fs::remove_file(format!("{}.gz", &old_js)),
            fs::remove_dir_all("frontend/pkg/snippets"),
        );
    }

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
    let success = Command::new("wasm-pack")
        .args(&args)
        .status()
        .context("Failed to get frontend build status")?
        .success();
    if success {
        let build_id = cache_busting
            .then(|| Uuid::new_v4().as_u128())
            .unwrap_or_default();

        let wasm_file_path = Path::new("frontend/pkg/frontend_bg.wasm");
        let new_wasm_file_path =
            PathBuf::from(format!("frontend/pkg/frontend_bg_{}.wasm", build_id));
        let js_file_path = Path::new("frontend/pkg/frontend.js");
        let new_js_file_path = PathBuf::from(format!("frontend/pkg/frontend_{}.js", build_id));

        try_join!(
            async { fs::rename(wasm_file_path, &new_wasm_file_path).await.context("Failed to rename the Wasm file in the pkg directory") },
            async { fs::rename(js_file_path, &new_js_file_path).await.context("Failed to rename the JS file in the pkg directory") },
            async { fs::write("frontend/pkg/build_id", build_id.to_string()).await.context("Failed to write the frontend build id") },
        ).map(|_|())?;

        if release {
            compress_pkg(&new_wasm_file_path, &new_js_file_path);
        }
        return Ok(())
    }
    bail!("Failed to build frontend")
}

pub fn compress_pkg(wasm_file_path: &Path, js_file_path: &Path) {
    compress_file(wasm_file_path);
    compress_file(js_file_path);

    visit_dirs(
        Path::new("frontend/pkg/snippets"),
        &mut |entry: &std::fs::DirEntry| {
            compress_file(&entry.path());
        },
    )
    .unwrap();
}

// @TODO refactor with https://crates.io/crates/async-compression
pub fn compress_file(file_path: &Path) {
    BrotliCompress(
        &mut std::fs::File::open(&file_path).unwrap(),
        &mut std::fs::File::create(&format!("{}.br", file_path.to_str().unwrap())).unwrap(),
        &BrotliEncoderParams::default(),
    )
    .unwrap();

    let file_reader = BufReader::new(std::fs::File::open(&file_path).unwrap());
    let mut gzip_encoder = GzEncoder::new(file_reader, Compression::best());
    let mut buffer = Vec::new();
    gzip_encoder.read_to_end(&mut buffer).unwrap();
    std::fs::write(&format!("{}.gz", file_path.to_str().unwrap()), buffer).unwrap();
}

pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&std::fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
