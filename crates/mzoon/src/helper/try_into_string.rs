use anyhow::{bail, Result};

pub trait TryIntoString {
    fn try_into_string(self) -> Result<String>;
}

impl TryIntoString for toml::Value {
    fn try_into_string(self) -> Result<String> {
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
                    .collect::<Result<Vec<_>>>()?
                    .join(",");
                format!("[{string_value}]")
            }
        };
        Ok(string_value)
    }
}
