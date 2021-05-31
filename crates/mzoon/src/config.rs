use serde::Deserialize;
use anyhow::{Context, Result};
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub https: bool,
    pub cache_busting: bool,
    pub redirect: Redirect,
    pub watch: Watch,
}

impl Config {
    pub async fn load_from_moonzoon_toml() -> Result<Config> {
        let toml = fs::read_to_string("MoonZoon.toml").await.context("Failed to read MoonZoon.toml")?;
        toml::from_str(&toml).context("Failed to parse MoonZoon.toml")
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
