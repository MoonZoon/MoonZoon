use moon::*;
use std::collections::BTreeSet;

// @TODO improve API & refactor the code below and:
// @TODO return `Result` from `from_env_vars`? +(rename?)
// @TODO write a derive macro `FromEnvVars`?

pub static CUSTOM_CONFIG: Lazy<CustomConfig> = Lazy::new(CustomConfig::from_env_vars);

#[derive(Debug, Deserialize)]
#[serde(crate = "serde")]
pub struct CustomConfig {
    pub my_api: String,
    pub pi: f32,
    pub favorite_languages: BTreeSet<String>,
    pub is_pig_pink: bool,
    pub birthday: DateTime<Local>,
    #[serde(default = "Postgres::from_env_vars")]
    pub postgres: Postgres
}

impl FromEnvVars for CustomConfig {
    const ENTITY_NAME: &'static str = "CustomConfig";
}

#[derive(Debug, Deserialize)]
#[serde(crate = "serde")]
pub struct Postgres {
    #[serde(default = "PostgresUser::from_env_vars")]
    pub user: PostgresUser,
    pub host: String,
    pub name: String,
}

impl FromEnvVars for Postgres {
    const ENTITY_NAME: &'static str = "Postgres";
    const ENV_PREFIX: &'static str = "POSTGRES_";
}

#[derive(Debug, Deserialize)]
#[serde(crate = "serde")]
pub struct PostgresUser {
    pub name: String,
    pub password: String,
}

impl FromEnvVars for PostgresUser {
    const ENTITY_NAME: &'static str = "PostgresUser";
    const ENV_PREFIX: &'static str = "POSTGRES_USER_";
}
