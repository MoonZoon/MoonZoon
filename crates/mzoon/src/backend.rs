use rcgen::{Certificate, CertificateParams};
use std::path::Path;
use std::process::{Command, Child};
use tokio::{fs, try_join, spawn};
use tokio::task::JoinHandle;
use tokio::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use anyhow::{bail, Context, Result};
use std::sync::Arc;
use parking_lot::Mutex;
use cargo_metadata::MetadataCommand;
use crate::config::Config;
use crate::project_watcher::ProjectWatcher;

pub async fn generate_certificate_if_not_present() -> Result<()> {
    let public_pem_path = Path::new("backend/private/public.pem");
    let private_pem_path = Path::new("backend/private/private.pem");
    if public_pem_path.is_file() && private_pem_path.is_file() {
        return Ok(());
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

    try_join!(
        async { fs::write(public_pem_path, public_pem).await.context("Failed to write the public key") },
        async { fs::write(private_pem_path, private_pem).await.context("Failed to write the private key") },
    ).map(|_| ())
}

pub fn start_backend_watcher(config: &Config, release: bool, debounce_time: Duration, server: Option<Child>) -> JoinHandle<Result<()>> {
    let paths = config.watch.backend.clone();

    spawn(async move {
        let project_watcher = ProjectWatcher::new(paths, debounce_time);
        let mut debounced_receiver = project_watcher.start().await?;

        let mut build_task = None::<JoinHandle<()>>;
        let server = Arc::new(Mutex::new(server));

        while debounced_receiver.recv().await.is_some() {
            println!("Build backend");
            if let Some(build_task) = build_task.take() {
                build_task.abort();
            }
            if let Some(mut server) = server.lock().take() {
                let _ = server.kill();
            }
            let server = Arc::clone(&server);
            build_task = Some(spawn(async move {
                match build_backend(release).await {
                    Ok(()) => {
                        match run_backend(release) {
                            Ok(backend) => { 
                                server.lock().replace(backend);
                            },
                            Err(error) => {
                                eprintln!("{}", error);
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }));
        }

        Ok(())
    })
}

pub async fn generate_backend_build_id() -> Result<()> {
    fs::write(
        "backend/private/build_id",
        Uuid::new_v4().as_u128().to_string(),
    )
    .await
    .context("Failed to write the backend build id")
}

pub fn run_backend(release: bool) -> Result<Child> {
    println!("Run backend");
    
    let mut target_directory = MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory;

    if release {
        target_directory.push("release")
    } else {
        target_directory.push("debug")
    };
    target_directory.push("backend");

    Command::new(target_directory).spawn().context("Failed to run backend")
}

pub async fn build_backend(release: bool) -> Result<()> {
    println!("Building backend...");

    let mut args = vec!["build", "--package", "backend"];
    if release {
        args.push("--release");
    }
    let success = Command::new("cargo")
        .args(&args)
        .status()
        .context("Failed to get frontend build status")?
        .success();
    if success {
        generate_backend_build_id().await?;
        return Ok(println!("Backend built"))
    }
    bail!("Failed to build backend")
}
