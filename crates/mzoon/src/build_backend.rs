use anyhow::{anyhow, Context, Error};
use apply::Apply;
use bool_ext::BoolExt;
use fehler::throws;
use rcgen::{Certificate, CertificateParams};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{fs, process::Command, try_join};
use uuid::Uuid;

// -- public --

#[throws]
pub async fn build_backend(release: bool, https: bool) {
    println!("Building backend...");

    if https {
        write_new_certificate_if_not_present().await?;
    }

    let mut args = vec!["build", "--package", "backend"];
    if release {
        args.push("--release");
    }
    Command::new("cargo")
        .args(&args)
        .status()
        .await
        .context("Failed to get frontend build status")?
        .success()
        .err(anyhow!("Failed to build backend"))?;

    write_new_build_id().await?;
    println!("Backend built");
}

// -- private --

#[throws]
async fn write_new_build_id() {
    fs::write(
        "backend/private/build_id",
        Uuid::new_v4().as_u128().to_string(),
    )
    .await
    .context("Failed to write the backend build id")?
}

#[throws]
async fn write_new_certificate_if_not_present() {
    let public_pem_path = Path::new("backend/private/public.pem");
    let private_pem_path = Path::new("backend/private/private.pem");
    if public_pem_path.is_file() && private_pem_path.is_file() {
        return;
    }
    let keys = generate_certificate();
    try_join!(
        async {
            fs::write(public_pem_path, &keys.public)
                .await
                .context("Failed to write the public key")
        },
        async {
            fs::write(private_pem_path, &keys.private)
                .await
                .context("Failed to write the private key")
        },
    )?
}

struct Keys {
    public: String,
    private: String,
}

fn generate_certificate() -> Keys {
    println!("Generate TLS certificate");

    let domains = vec!["localhost".to_owned()];
    let mut params = CertificateParams::new(domains);

    // https://support.mozilla.org/en-US/kb/Certificate-contains-the-same-serial-number-as-another-certificate
    params.serial_number = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .apply(Some);

    let certificate = Certificate::from_params(params).unwrap();
    Keys {
        public: certificate.serialize_pem().unwrap(),
        private: certificate.serialize_private_key_pem(),
    }
}
