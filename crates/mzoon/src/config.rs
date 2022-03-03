use anyhow::{Context, Error, bail};
use fehler::throws;
use log::LevelFilter;
use serde::Deserialize;
use tokio::fs;
use std::{collections::VecDeque, rc::Rc};
use crate::helper::TryIntoString;

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

type Name = String;

enum NodeContent<T> {
    Children(Box<dyn Iterator<Item = (Name, T)>>),
    Value(String),
}

// #[throws]
fn toml_to_env_vars(toml: toml::Value) -> anyhow::Result<Vec<(String, String)>> {
    println!("{toml:#}");
    tree_to_string_pairs(
        toml,
        |parent_name, original_name| format!("{parent_name}_{original_name}"),
        |toml| {
            match toml {
                toml::Value::Table(table) => {
                    Ok(NodeContent::Children(Box::new(
                        table.into_iter().map(|(name, value)| (name.to_ascii_uppercase(), value))
                    )))
                }
                value => Ok(NodeContent::Value(value.try_into_string()?))
            }
        },
    )
}

fn tree_to_string_pairs<T>(
    tree: T,
    child_name: impl Fn(&str, &str) -> Name,
    children_or_value: impl Fn(T) -> anyhow::Result<NodeContent<T>>,
) -> anyhow::Result<Vec<(Name, String)>> 
{
    let mut vars = Vec::<(Name, String)>::new();

    struct StackItem<T> {
        parent_name: Option<Rc<Name>>,
        name: Option<Name>,
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
            NodeContent::Children(children) => {
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
            NodeContent::Value(value) => value,
        }; 
        if let Some(output_name) = output_name {
            vars.push((output_name, output_value));
        } else {
            bail!("Root node cannot be a leaf node")
        }
    }
    Ok(vars)
}
