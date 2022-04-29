use crate::*;

pub trait HasLang: UpdateRawEl + Sized {
    fn lang<'a>(self, lang: impl IntoCowStr<'a>) -> Self {
        self.update_raw_el(move |raw_el| raw_el.lang(lang))
    }
}