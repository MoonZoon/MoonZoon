use std::env;
use crate::config::Config;

pub fn set_env_vars(config: &Config, release: bool) {
    // port = 8443
    env::set_var("PORT", config.port.to_string());
    // https = true
    env::set_var("HTTPS", config.https.to_string());
    // backend_log_level = "warn"
    env::set_var("BACKEND_LOG_LEVEL", config.backend_log_level.as_str());

    // [redirect]
    // port = 8080
    env::set_var("REDIRECT_PORT", config.redirect.port.to_string());
    // enabled = true
    env::set_var("REDIRECT_ENABLED", config.redirect.enabled.to_string());

    env::set_var("COMPRESSED_PKG", release.to_string());
}
