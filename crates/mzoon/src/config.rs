use anyhow::{Context, Error, bail};
use fehler::throws;
use log::LevelFilter;
use serde::Deserialize;
use tokio::fs;
use std::collections::VecDeque;
use std::rc::Rc;

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

impl Config {
    #[throws]
    pub async fn load_from_moonzoon_tomls() -> Config {
        let config_toml = fs::read_to_string("MoonZoon.toml")
            .await
            .context("Failed to read MoonZoon.toml")?;
        let mut config = toml::from_str(&config_toml).context("Failed to parse MoonZoon.toml")?;

        if fs::metadata("MoonZoonCustom.toml").await.is_err() {
            return config;
        }
        let custom_config_toml = fs::read_to_string("MoonZoonCustom.toml")
            .await
            .context("Failed to read MoonZoonCustom.toml")?;
        let custom_config = toml::from_str(&custom_config_toml).context("Failed to parse MoonZoonCustom.toml")?;
        config.custom_env_vars = toml_to_env_vars(custom_config).context("Failed to parse MoonZoonCustom.toml's content")?;
        config
    }
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

// #[throws]
fn toml_to_env_vars(toml: toml::Value) -> anyhow::Result<Vec<(String, String)>> {
    println!("{toml:#}");

    let mut vars = Vec::<(String, String)>::new();

    struct StackItem {
        parent_name: Option<Rc<String>>,
        name: Option<String>,
        toml_value: toml::Value
    }

    let mut stack = VecDeque::<StackItem>::new();
    let root = StackItem {
        parent_name: None,
        name: None,
        toml_value: toml
    };
    stack.push_back(root);

    while let Some(StackItem { parent_name, name, toml_value }) = stack.pop_front() {
        let env_name = match (parent_name, name) {
            (Some(parent_name), Some(name)) => {
                Some(format!("{parent_name}_{name}"))
            }
            (None, Some(name)) => Some(name),
            (None, None) => None,
            (Some(_), None) => unreachable!(),
        };
        let env_value = match toml_value {
            toml::Value::Table(table) => {
                let parent_name = env_name.map(Rc::new);
                stack.extend(table.into_iter().map(|(name, toml_value)| {
                    StackItem { 
                        parent_name: parent_name.clone(), 
                        name: Some(name.to_ascii_uppercase()), 
                        toml_value 
                    }
                }));
                continue;
            }
            value => value.try_into_string()?
        };
        if let Some(env_name) = env_name {
            vars.push((env_name, env_value));
        } else {
            unreachable!();
        }
    }

    println!("{vars:#?}");
    Ok(vars)
}

trait TryIntoString {
    fn try_into_string(self) -> anyhow::Result<String>;
}

impl TryIntoString for toml::Value {
    fn try_into_string(self) -> anyhow::Result<String> {
        let string_value = match self {
            toml::Value::Table(_) => bail!("TOML tables cannot be stringified"),
            toml::Value::Boolean(value) => value.to_string(),
            toml::Value::Float(value) => value.to_string(),
            toml::Value::Integer(value) => value.to_string(),
            toml::Value::String(value) => value,
            toml::Value::Datetime(value) => value.to_string(),
            toml::Value::Array(value) => {
                let string_value = value
                    .into_iter()
                    .map(|value| value.try_into_string())
                    .collect::<anyhow::Result<Vec<_>>>()?
                    .join(",");
                format!("[{string_value}]")
            },
        };
        Ok(string_value)
    }
}
