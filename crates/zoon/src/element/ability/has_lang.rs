use crate::*;

// ------ HasLang ------

pub trait HasLang: UpdateRawEl + Sized {
    fn lang<'a>(self, lang: Lang) -> Self {
        self.update_raw_el(move |raw_el| raw_el.lang(lang))
    }
}

// ------ Lang ------

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
