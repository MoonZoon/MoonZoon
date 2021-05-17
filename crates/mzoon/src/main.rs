use brotli::{enc::backward_references::BrotliEncoderParams, BrotliCompress};
use flate2::bufread::GzEncoder;
use flate2::Compression;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rcgen::{Certificate, CertificateParams};
use serde::Deserialize;
use std::env;
use std::fs::{self, DirEntry, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
enum Opt {
    New {
        project_name: String,
    },
    Start {
        #[structopt(short, long)]
        release: bool,
        // #[structopt(short, long)]
        // open: bool
    },
    Build {
        #[structopt(short, long)]
        release: bool,
    },
}

// Run from example:  cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start

fn main() {
    let (frontend_sender, frontend_receiver) = channel();
    let (backend_sender, backend_receiver) = channel();
    ctrlc::set_handler({
        let frontend_sender = frontend_sender.clone();
        let backend_sender = backend_sender.clone();
        move || {
            println!("Shut down MoonZoon CLI");
            let event = || DebouncedEvent::Error(notify::Error::Generic("ctrl-c".to_owned()), None);
            frontend_sender.send(event()).unwrap();
            backend_sender.send(event()).unwrap();
        }
    })
    .unwrap();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt {
        Opt::New { .. } => {}
        Opt::Start { release } => {
            let config = load_config();
            set_env_vars(&config, release);

            check_wasm_pack();
            if config.https {
                generate_certificate();
            }

            let (frontend_build_finished_sender, frontend_build_finished_receiver) = channel();
            let frontend_watcher_handle = start_frontend_watcher(
                config.watch.frontend.clone(),
                release,
                frontend_sender,
                frontend_receiver,
                frontend_build_finished_sender,
                &config,
            );
            // @TODO parallel build instead of waiting (server has to be started after FE build!)
            // `recv` fails if the sender is dropped because of fail in `start_frontend_watcher`
            let _ = frontend_build_finished_receiver.recv();

            let backend_watcher_handle = start_backend_watcher(
                config.watch.backend.clone(),
                release,
                // open,
                backend_sender,
                backend_receiver,
            );
            frontend_watcher_handle.join().unwrap();
            backend_watcher_handle.join().unwrap();
        }
        Opt::Build { release } => {
            let config = load_config();
            set_env_vars(&config, release);

            check_wasm_pack();
            if config.https {
                generate_certificate();
            }

            if !build_frontend(release, config.cache_busting) {
                panic!("Build frontend failed!");
            }

            if !build_backend(release) {
                panic!("Build backend failed!");
            }
        }
    }
}

fn load_config() -> Config {
    let toml = fs::read_to_string("MoonZoon.toml").unwrap();
    toml::from_str(&toml).unwrap()
}

fn set_env_vars(config: &Config, release: bool) {
    // port = 8443
    env::set_var("PORT", config.port.to_string());
    // https = true
    env::set_var("HTTPS", config.https.to_string());

    // [redirect_server]
    // port = 8080
    env::set_var(
        "REDIRECT_SERVER__PORT",
        config.redirect_server.port.to_string(),
    );
    // enabled = true
    env::set_var(
        "REDIRECT_SERVER__ENABLED",
        config.redirect_server.enabled.to_string(),
    );

    env::set_var("COMPRESSED_PKG", release.to_string());
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    https: bool,
    cache_busting: bool,
    redirect_server: RedirectServer,
    watch: Watch,
}

#[derive(Debug, Deserialize)]
struct RedirectServer {
    port: u16,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct Watch {
    frontend: Vec<String>,
    backend: Vec<String>,
}

fn check_wasm_pack() {
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

fn generate_certificate() {
    let public_pem_path = Path::new("backend/private/public.pem");
    let private_pem_path = Path::new("backend/private/private.pem");
    if public_pem_path.is_file() && private_pem_path.is_file() {
        return;
    }
    println!("Generate TLS certificate");

    let domains = vec!["localhost".to_owned()];
    let mut params = CertificateParams::new(domains);

    // https://support.mozilla.org/en-US/kb/Certificate-contains-the-same-serial-number-as-another-certificate
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    params.serial_number = Some(since_the_epoch);

    let certificate = Certificate::from_params(params).unwrap();

    let public_pem = certificate.serialize_pem().unwrap();
    let private_pem = certificate.serialize_private_key_pem();
    fs::write(public_pem_path, public_pem).unwrap();
    fs::write(private_pem_path, private_pem).unwrap();
}

fn start_frontend_watcher(
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

enum BackendCommand {
    Rebuild,
    Stop,
}

fn start_backend_watcher(
    paths: Vec<String>,
    release: bool,
    // open: bool,
    sender: Sender<DebouncedEvent>,
    receiver: Receiver<DebouncedEvent>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut watcher = watcher(sender, Duration::from_millis(100)).unwrap();
        for path in paths {
            watcher.watch(&path, RecursiveMode::Recursive).unwrap();
        }

        let (server_rebuild_run_sender, server_rebuild_run_receiver) = channel();
        let backend_handle = thread::spawn(move || {
            // let mut open = open;
            loop {
                // @TODO only on successful build
                generate_backend_build_id();
                let mut cargo_and_server_process = build_and_run_backend(release);
                // @TODO wait for (successful) build
                // if open {
                //     open = false;
                //     let address = "https://127.0.0.1:8443";
                //     println!("Open {} in your default web browser", "https://127.0.0.1:8443");
                //     open::that(address).unwrap();
                // }
                let command = server_rebuild_run_receiver.recv();
                match command {
                    Ok(BackendCommand::Rebuild) => {
                        let _ = cargo_and_server_process.kill();
                    }
                    Ok(BackendCommand::Stop) => {
                        cargo_and_server_process.wait().unwrap();
                        break;
                    }
                    Err(error) => {
                        println!("watch backend error: {:?}", error);
                        break;
                    }
                }
            }
        });

        loop {
            match receiver.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeWrite(_) | DebouncedEvent::NoticeRemove(_) => (),
                    DebouncedEvent::Error(notify::Error::Generic(error), _)
                        if error == "ctrl-c" =>
                    {
                        let _ = server_rebuild_run_sender.send(BackendCommand::Stop);
                        backend_handle.join().unwrap();
                        return;
                    }
                    _ => {
                        println!("Build backend");
                        if server_rebuild_run_sender
                            .send(BackendCommand::Rebuild)
                            .is_err()
                        {
                            return;
                        }
                    }
                },
                Err(error) => {
                    println!("watch backend error: {:?}", error);
                    return;
                }
            }
        }
    })
}

fn generate_backend_build_id() {
    fs::write(
        "backend/private/build_id",
        Uuid::new_v4().as_u128().to_string(),
    )
    .unwrap();
}

fn build_frontend(release: bool, cache_busting: bool) -> bool {
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

fn compress_pkg(wasm_file_path: &Path, js_file_path: &Path) {
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
fn compress_file(file_path: &Path) {
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

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
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

fn build_and_run_backend(release: bool) -> Child {
    let mut args = vec!["run", "--package", "backend"];
    if release {
        args.push("--release");
    }
    Command::new("cargo").args(&args).spawn().unwrap()
}

fn build_backend(release: bool) -> bool {
    let mut args = vec!["build", "--package", "backend"];
    if release {
        args.push("--release");
    }
    let success = Command::new("cargo")
        .args(&args)
        .status()
        .unwrap()
        .success();
    if success {
        generate_backend_build_id();
    }
    success
}
