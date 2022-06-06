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

    // [redirect]
    // port = 8080
    env::set_var("REDIRECT_PORT", config.redirect.port.to_string());
    // enabled = true
    env::set_var("REDIRECT_ENABLED", config.redirect.enabled.to_string());

    // [cors]
    // origins = ["*", "https://example.com"]
    env::set_var("CORS_ORIGINS", config.cors.origins.join(","));

    env::set_var("COMPRESSED_PKG", build_mode.is_not_dev().to_string());

    env::set_var("FRONTEND_DIST", frontend_dist.to_string());

    for (key, value) in &config.custom_env_vars {
        env::set_var(key, value);
    }
}
