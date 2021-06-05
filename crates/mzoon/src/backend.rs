use rcgen::{Certificate, CertificateParams};
use std::path::Path;
use std::process::{Command, Child};
use tokio::{fs, try_join};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use anyhow::{anyhow, Context, Error};
use fehler::throws;
use cargo_metadata::MetadataCommand;

// -- public --

#[throws]
pub async fn build_backend(release: bool, https: bool) {
    println!("Building backend...");

    if https {
        generate_certificate_if_not_present().await?;
    }

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
        return println!("Backend built");
    }
    Err(anyhow!("Failed to build backend"))?;
}

#[throws]
pub fn run_backend(release: bool) -> Child {
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

    Command::new(target_directory).spawn().context("Failed to run backend")?
}

// -- private --

#[throws]
async fn generate_backend_build_id() {
    fs::write(
        "backend/private/build_id",
        Uuid::new_v4().as_u128().to_string(),
    )
    .await
    .context("Failed to write the backend build id")?
}

#[throws]
async fn generate_certificate_if_not_present() {
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

    try_join!(
        async { fs::write(public_pem_path, public_pem).await.context("Failed to write the public key") },
        async { fs::write(private_pem_path, private_pem).await.context("Failed to write the private key") },
    )?
}
