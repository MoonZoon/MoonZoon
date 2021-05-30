use brotli::{enc::backward_references::BrotliEncoderParams, BrotliCompress};
use flate2::bufread::GzEncoder;
use flate2::Compression;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::{self, DirEntry, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use uuid::Uuid;
use crate::config::Config;

pub fn check_wasm_pack() {
    let status = Command::new("wasm-pack")
        .args(&["-V"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) if status.success() => (),
        _ => panic!("Cannot find `wasm-pack`! Please install it by `cargo install wasm-pack` or download/install pre-built binaries into a globally available directory."),
    }
}

pub fn start_frontend_watcher(
    paths: Vec<String>,
    release: bool,
    sender: Sender<DebouncedEvent>,
    receiver: Receiver<DebouncedEvent>,
    frontend_build_finished_sender: Sender<()>,
    config: &Config,
) -> JoinHandle<()> {
    let reload_url = format!(
        "{protocol}://localhost:{port}/api/reload",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    );
    let cache_busting = config.cache_busting;

    thread::spawn(move || {
        let mut watcher = watcher(sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }
        build_frontend(release, cache_busting);
        frontend_build_finished_sender.send(()).unwrap();
        loop {
            match receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent::NoticeRemove(_) => (),
                    DebouncedEvent::Error(notify::Error::Generic(error), _)
                        if error == "ctrl-c" =>
                    {
                        break
                    }
                    _ => {
                        println!("Build frontend");
                        if build_frontend(release, cache_busting) {
                            println!("Reload frontend");
                            attohttpc::post(&reload_url)
                                .danger_accept_invalid_certs(true)
                                .send()
                                .unwrap();
                        }
                    }
                },
                Err(error) => panic!("watch frontend error: {:?}", error),
            }
        }
    })
}

pub fn build_frontend(release: bool, cache_busting: bool) -> bool {
    let old_build_id = fs::read_to_string("frontend/pkg/build_id")
        .ok()
        .map(|uuid| uuid.parse::<u128>().map(|uuid| uuid).unwrap_or_default());
    if let Some(old_build_id) = old_build_id {
        let old_wasm = format!("frontend/pkg/frontend_bg_{}.wasm", old_build_id);
        let old_js = format!("frontend/pkg/frontend_{}.js", old_build_id);
        let _ = fs::remove_file(&old_wasm);
        let _ = fs::remove_file(&old_js);
        let _ = fs::remove_file(format!("{}.br", &old_wasm));
        let _ = fs::remove_file(format!("{}.br", &old_js));
        let _ = fs::remove_file(format!("{}.gz", &old_wasm));
        let _ = fs::remove_file(format!("{}.gz", &old_js));
        // @TODO replace with the crate with more reliable removing on Windows?
        let _ = fs::remove_dir_all("frontend/pkg/snippets");
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
        .unwrap()
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

        fs::rename(wasm_file_path, &new_wasm_file_path).unwrap();
        fs::rename(js_file_path, &new_js_file_path).unwrap();
        fs::write("frontend/pkg/build_id", build_id.to_string()).unwrap();

        if release {
            compress_pkg(&new_wasm_file_path, &new_js_file_path);
        }
    }
    success
}

pub fn compress_pkg(wasm_file_path: &Path, js_file_path: &Path) {
    compress_file(wasm_file_path);
    compress_file(js_file_path);

    visit_dirs(
        Path::new("frontend/pkg/snippets"),
        &mut |entry: &DirEntry| {
            compress_file(&entry.path());
        },
    )
    .unwrap();
}

// @TODO refactor with https://crates.io/crates/async-compression
pub fn compress_file(file_path: &Path) {
    BrotliCompress(
        &mut File::open(&file_path).unwrap(),
        &mut File::create(&format!("{}.br", file_path.to_str().unwrap())).unwrap(),
        &BrotliEncoderParams::default(),
    )
    .unwrap();

    let file_reader = BufReader::new(File::open(&file_path).unwrap());
    let mut gzip_encoder = GzEncoder::new(file_reader, Compression::best());
    let mut buffer = Vec::new();
    gzip_encoder.read_to_end(&mut buffer).unwrap();
    fs::write(&format!("{}.gz", file_path.to_str().unwrap()), buffer).unwrap();
}

pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
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
