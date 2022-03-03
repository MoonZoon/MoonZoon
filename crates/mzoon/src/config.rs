use anyhow::{Context, Error};
use fehler::throws;
use log::LevelFilter;
use serde::Deserialize;
use tokio::fs;
use crate::helper::{TryIntoString, tree_into_pairs::{tree_into_pairs, NodeContent}};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub https: bool,
    pub cache_busting: bool,
    pub backend_log_level: LevelFilter,
    pub redirect: Redirect,
    pub cors: Cors,
    pub watch: Watch,
    #[serde(skip)]
    pub custom_env_vars: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Cors {
    pub origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Watch {
    pub frontend: Vec<String>,
    pub backend: Vec<String>,
}

impl Config {
    #[throws]
    pub async fn load_from_moonzoon_tomls() -> Config {
        let mut config = read_moonzoon_toml().await?;
        if let Some(custom_env_vars) = read_moonzoon_custom_toml().await? {
            println!("custom_env_vars: {custom_env_vars:#?}");
            config.custom_env_vars = custom_env_vars;
        }
        config
    }
}

#[throws]
async fn read_moonzoon_toml() -> Config {
    let config_toml = fs::read_to_string("MoonZoon.toml")
        .await
        .context("Failed to read MoonZoon.toml")?;

    toml::from_str(&config_toml).context("Failed to parse MoonZoon.toml")?
}

#[throws]
async fn read_moonzoon_custom_toml() -> Option<Vec<(String, String)>> {
    if fs::metadata("MoonZoonCustom.toml").await.is_err() {
        return None;
    }

    let custom_config_toml = fs::read_to_string("MoonZoonCustom.toml")
        .await
        .context("Failed to read MoonZoonCustom.toml")?;

    let custom_config = toml::from_str(&custom_config_toml).context("Failed to parse MoonZoonCustom.toml")?;
    let pairs = toml_to_env_vars(custom_config).context("Failed to parse MoonZoonCustom.toml's content")?;
    Some(pairs)
} 

#[throws]
fn toml_to_env_vars(toml: toml::Value) -> Vec<(String, String)> {
    tree_into_pairs(
        toml,
        |parent_name, original_name| format!("{parent_name}_{original_name}"),
        |toml| {
            match toml {
                toml::Value::Table(table) => {
                    Ok(NodeContent::Children(Box::new(
                        table.into_iter().map(|(mut name, value)| {
                            name.make_ascii_uppercase();
                            (name, value)
                        })
                    )))
                }
                value => Ok(NodeContent::Value(value.try_into_string()?))
            }
        },
    )?
}
