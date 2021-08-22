use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(token: impl ToString) -> Self {
        AuthToken(token.to_string())
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
