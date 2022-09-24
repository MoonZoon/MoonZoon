use anyhow::Error;
use fehler::throws;
use std::path::PathBuf;
use tar::Archive;
use tokio::{fs, task};

static NEW_PROJECT_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/new_project.tar"));

#[throws]
pub async fn new(path: PathBuf, local_deps: bool) {
    task::spawn_blocking({
        let path = path.clone();
        || Archive::new(NEW_PROJECT_TAR).unpack(path)
    })
    .await??;
    postprocess_project_files(path, local_deps).await?;
    println!("New project created");
}

#[throws]
pub async fn postprocess_project_files(path: PathBuf, local_deps: bool) {
    if !local_deps {
        let replacements = [
            (
                r#"moon = { path = "../../moon" }"#,
                r#"moon = { git = "https://github.com/MoonZoon/MoonZoon", branch = "main" }"#,
            ),
            (
                r#"zoon = { path = "../../zoon" }"#,
                r#"zoon = { git = "https://github.com/MoonZoon/MoonZoon", branch = "main" }"#,
            ),
        ];
        replace_in_file(path.join("Cargo.toml"), replacements).await?;
    }
}

#[throws]
pub async fn replace_in_file(path: PathBuf, replacements: impl IntoIterator<Item = (&str, &str)>) {
    let mut content = fs::read_to_string(&path).await?;
    for (from, to) in replacements {
        content = content.replace(from, to);
    }
    fs::write(path, content).await?;
}
