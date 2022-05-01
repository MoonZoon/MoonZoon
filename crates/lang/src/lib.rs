// ------ Lang ------

use std::fmt;

#[derive(Copy, Clone)]
pub enum Lang<'a> {
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
            Lang::English => "en",
            Lang::French => "fr",
            Lang::Norwegian => "no",
            Lang::Spanish => "es",
            Lang::Swedish => "sv",
            Lang::Custom(lang) => lang,
        }
    }
}

impl<'a> fmt::Display for Lang<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
