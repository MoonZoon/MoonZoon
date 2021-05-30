use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rcgen::{Certificate, CertificateParams};
use std::fs;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn generate_certificate() {
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

pub enum BackendCommand {
    Rebuild,
    Stop,
}

pub fn start_backend_watcher(
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

pub fn generate_backend_build_id() {
    fs::write(
        "backend/private/build_id",
        Uuid::new_v4().as_u128().to_string(),
    )
    .unwrap();
}

pub fn build_and_run_backend(release: bool) -> Child {
    let mut args = vec!["run", "--package", "backend"];
    if release {
        args.push("--release");
    }
    Command::new("cargo").args(&args).spawn().unwrap()
}

pub fn build_backend(release: bool) -> bool {
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
