use crate::{config::Config, BuildMode};
use std::env;

pub fn set_env_vars(config: &Config, build_mode: BuildMode, frontend_dist: bool) {
    // port = 8443
    env::set_var("PORT", config.port.to_string());
    // https = true
    env::set_var("HTTPS", config.https.to_string());
    // cache_busting = true
    env::set_var("CACHE_BUSTING", config.cache_busting.to_string());
    // backend_log_level = "warn"
    env::set_var("BACKEND_LOG_LEVEL", config.backend_log_level.as_str());
    // frontend_multithreading = true
    env::set_var(
        "FRONTEND_MULTITHREADING",
        (config.frontend_multithreading == Some(true)).to_string(),
    );

    // [redirect]
    // port = 8080
    env::set_var("REDIRECT_PORT", config.redirect.port.to_string());
    // enabled = true
    env::set_var("REDIRECT_ENABLED", config.redirect.enabled.to_string());

    // [cors]
    // origins = ["*", "https://example.com"]
    env::set_var("CORS_ORIGINS", config.cors.origins.join(","));

    env::set_var(
        "COMPRESSED_PKG",
        (build_mode.is_not_dev() && !frontend_dist).to_string(),
    );

    // frontend_dist = false
    env::set_var("FRONTEND_DIST", frontend_dist.to_string());

    // frontend_auto_reload = false
    env::set_var(
        "FRONTEND_AUTO_RELOAD",
        (build_mode.is_not_release() && !frontend_dist).to_string(),
    );

    // custom configs from MoonZoonCustom.toml
    for (key, value) in &config.custom_env_vars {
        env::set_var(key, value);
    }
}
