use anyhow::{Context, Error, bail};
use fehler::throws;
use log::LevelFilter;
use serde::Deserialize;
use tokio::fs;
use std::{collections::VecDeque, rc::Rc, ops::ControlFlow};

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
        println!("{:#?}", config.custom_env_vars);
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

type NodeChildren<T> = Box<dyn Iterator<Item = (String, T)>>;

// #[throws]
fn toml_to_env_vars(toml: toml::Value) -> anyhow::Result<Vec<(String, String)>> {
    println!("{toml:#}");
    tree_to_string_pairs(
        toml,
        |parent_name, original_name| format!("{parent_name}_{original_name}"),
        |toml| {
            match toml {
                toml::Value::Table(table) => {
                    Ok(ControlFlow::Continue(Box::new(
                        table.into_iter().map(|(name, value)| (name.to_ascii_uppercase(), value))
                    )))
                }
                value => Ok(ControlFlow::Break(value.try_into_string()?))
            }
        },
    )
}

fn tree_to_string_pairs<T>(
    tree: T,
    child_name: impl Fn(&str, &str) -> String,
    children_or_value: impl Fn(T) -> anyhow::Result<ControlFlow<String, NodeChildren<T>>>,
) -> anyhow::Result<Vec<(String, String)>> 
{
    let mut vars = Vec::<(String, String)>::new();

    struct StackItem<T> {
        parent_name: Option<Rc<String>>,
        name: Option<String>,
        node: T,
    }

    let mut stack = VecDeque::<StackItem<T>>::new();
    let root = StackItem {
        parent_name: None,
        name: None,
        node: tree,
    };
    stack.push_back(root);

    while let Some(StackItem { parent_name, name, node }) = stack.pop_front() {
        let output_name = match (parent_name, name) {
            (Some(parent_name), Some(name)) => {
                Some(child_name(&parent_name, &name))
            }
            (None, Some(name)) => Some(name),
            (None, None) => None,
            (Some(_), None) => unreachable!(),
        };
        let output_value = match children_or_value(node)? {
            ControlFlow::Continue(children) => {
                let parent_name = output_name.map(Rc::new);
                stack.extend(children.map(|(name, node)| {
                    StackItem { 
                        parent_name: parent_name.clone(), 
                        name: Some(name), 
                        node 
                    }
                }));
                continue;
            },
            ControlFlow::Break(value) => value,
        }; 
        if let Some(output_name) = output_name {
            vars.push((output_name, output_value));
        } else {
            unreachable!();
        }
    }
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
