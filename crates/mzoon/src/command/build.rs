use crate::build_backend::build_backend;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use crate::frontend_dist::create_frontend_dist;
use crate::set_env_vars::set_env_vars;
use crate::BuildMode;
use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn build(build_mode: BuildMode, frontend_dist: bool) {
    let config = Config::load_from_moonzoon_tomls().await?;
    set_env_vars(&config, build_mode, frontend_dist);

    build_frontend(build_mode, config.cache_busting, frontend_dist).await?;
    build_backend(build_mode, config.https).await?;

    if frontend_dist {
        create_frontend_dist(build_mode, &config).await?;
    }
}
