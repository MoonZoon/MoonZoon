use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

// ------ Lang ------
// @TODO: Add remaining langs
// @TODO optional `serde`?
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum Lang {
    Czech,
    #[default]
    English,
    French,
    // @TODO `no` vs `nb` vs `ny`
    Norwegian,
    Spanish,
    Swedish,
    Custom(Cow<'static, str>),
}

impl Lang {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Czech => "cs",
            Self::English => "en",
            Self::French => "fr",
            Self::Norwegian => "no",
            Self::Spanish => "es",
            Self::Swedish => "sv",
            Self::Custom(lang) => lang,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
