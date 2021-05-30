use serde::Deserialize;
use std::fs;

pub fn load_config() -> Config {
    let toml = fs::read_to_string("MoonZoon.toml").unwrap();
    toml::from_str(&toml).unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub https: bool,
    pub cache_busting: bool,
    pub redirect: Redirect,
    pub watch: Watch,
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
