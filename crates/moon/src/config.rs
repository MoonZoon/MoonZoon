use serde::Deserialize;
use crate::from_env_vars::FromEnvVars;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    // PORT
    pub port: u16,
    // HTTPS
    pub https: bool,
    // COMPRESSED_PKG
    pub compressed_pkg: bool,

    #[serde(default = "RedirectServer::from_env_vars")]
    pub redirect_server: RedirectServer,
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
            redirect_server: RedirectServer::default()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RedirectServer {
    // REDIRECT_SERVER__PORT
    pub port: u16,
    // REDIRECT_SERVER__ENABLED
    pub enabled: bool,
}

impl FromEnvVars for RedirectServer {
    const ENTITY_NAME: &'static str = "RedirectServer";
    const ENV_PREFIX: &'static str = "REDIRECT_SERVER__";
}

impl Default for RedirectServer {
    fn default() -> Self {
        Self {
            port: 8081,
            enabled: false,
        }
    }
}
