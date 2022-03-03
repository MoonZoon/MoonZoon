use moon::*;
use std::collections::BTreeSet;

// @TODO return `Result` from `from_env_vars`?
// @TODO write a derive macro `FromEnvVars`?

pub static CUSTOM_CONFIG: Lazy<CustomConfig> = Lazy::new(CustomConfig::from_env_vars);

#[derive(Debug, Deserialize, Default)]
#[serde(crate = "serde")]
pub struct CustomConfig {
    pub my_api: String,
    pub pi: f32,
    pub favorite_languages: BTreeSet<String>,
}

impl FromEnvVars for CustomConfig {
    const ENTITY_NAME: &'static str = "CustomConfig";
}

// #[derive(Debug, Deserialize)]
// #[serde(default)]
// pub struct Redirect {
//     // REDIRECT_PORT
//     pub port: u16,
//     // REDIRECT_ENABLED
//     pub enabled: bool,
// }

// impl FromEnvVars for Redirect {
//     const ENTITY_NAME: &'static str = "Redirect";
//     const ENV_PREFIX: &'static str = "REDIRECT_";
// }

// impl Default for Redirect {
//     fn default() -> Self {
//         Self {
//             port: 8081,
//             enabled: false,
//         }
//     }
// }

// #[derive(Debug, Deserialize)]
// #[serde(default)]
// pub struct Cors {
//     // CORS_ORIGINS="http://localhost:8080,http://127.0.0.1:8080,*"
//     pub origins: BTreeSet<Cow<'static, str>>,
// }

// impl FromEnvVars for Cors {
//     const ENTITY_NAME: &'static str = "Cors";
//     const ENV_PREFIX: &'static str = "CORS_";
// }

// impl Default for Cors {
//     fn default() -> Self {
//         Self {
//             origins: BTreeSet::from_iter(["*".into()]),
//         }
//     }
// }
