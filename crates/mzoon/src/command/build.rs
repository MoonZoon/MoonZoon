use crate::build_backend::build_backend;
use crate::build_frontend::build_frontend;
use crate::config::Config;
use crate::set_env_vars::set_env_vars;
use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn build(release: bool) {
    let config = Config::load_from_moonzoon_tomls().await?;
    set_env_vars(&config, release);

    build_frontend(release, config.cache_busting).await?;
    build_backend(release, config.https).await?;
}
