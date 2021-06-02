use anyhow::Result;
use crate::config::Config;
use crate::frontend::build_frontend;
use crate::backend::build_backend;
use crate::set_env_vars::set_env_vars;

pub async fn build(release: bool) -> Result<()> {
    let config = Config::load_from_moonzoon_toml().await?;
    set_env_vars(&config, release);

    build_frontend(release, config.cache_busting).await?;
    build_backend(release, config.https).await?;
    Ok(())
}
