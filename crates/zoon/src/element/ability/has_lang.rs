use crate::*;
use lang::Lang;

// ------ HasLang ------

pub trait HasLang: UpdateRawEl + Sized {
    fn lang<'a>(self, lang: Lang) -> Self {
        self.update_raw_el(move |raw_el| raw_el.lang(lang))
    }
}
