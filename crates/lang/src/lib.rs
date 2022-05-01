// ------ Lang ------

use std::fmt;

#[derive(Copy, Clone)]
pub enum Lang<'a> {
    Czech,
    English,
    French,
    Norwegian,
    Spanish,
    Swedish,
    Custom(&'a str),
}

impl<'a> Lang<'a> {
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

impl<'a> fmt::Display for Lang<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
