use serde::Deserialize;

pub trait FromEnvVars
where
    for<'de> Self: Deserialize<'de>,
{
    const ENTITY_NAME: &'static str;
    const ENV_PREFIX: &'static str = "";

    fn from_env_vars() -> Self {
        envy::prefixed(Self::ENV_PREFIX)
            .from_env()
            .unwrap_or_else(|error| {
                panic!(
                    "cannot load {} from env variables: {}",
                    Self::ENTITY_NAME,
                    error
                )
            })
    }
}
