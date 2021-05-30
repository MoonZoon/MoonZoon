use crate::from_env_vars::FromEnvVars;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    // PORT
    pub port: u16,
    // HTTPS
    pub https: bool,
    // COMPRESSED_PKG
    pub compressed_pkg: bool,

    #[serde(default = "Redirect::from_env_vars")]
    pub redirect: Redirect,
}

impl FromEnvVars for Config {
    const ENTITY_NAME: &'static str = "Config";
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            https: false,
            compressed_pkg: true,
            redirect: Redirect::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Redirect {
    // REDIRECT_PORT
    pub port: u16,
    // REDIRECT_ENABLED
    pub enabled: bool,
}

impl FromEnvVars for Redirect {
    const ENTITY_NAME: &'static str = "Redirect";
    const ENV_PREFIX: &'static str = "REDIRECT_";
}

impl Default for Redirect {
    fn default() -> Self {
        Self {
            port: 8081,
            enabled: false,
        }
    }
}
