use std::{borrow::Cow, fmt};

// ------ Lang ------
// @TODO: Add remaining langs
#[derive(Clone)]
pub enum Lang {
    Czech,
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
