use anyhow::{Context, Error};
use fehler::throws;
use log::LevelFilter;
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub https: bool,
    pub cache_busting: bool,
    pub backend_log_level: LevelFilter,
    pub redirect: Redirect,
    pub watch: Watch,
}

impl Config {
    #[throws]
    pub async fn load_from_moonzoon_toml() -> Config {
        let toml = fs::read_to_string("MoonZoon.toml")
            .await
            .context("Failed to read MoonZoon.toml")?;
        toml::from_str(&toml).context("Failed to parse MoonZoon.toml")?
    }
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Watch {
    pub frontend: Vec<String>,
    pub backend: Vec<String>,
}
