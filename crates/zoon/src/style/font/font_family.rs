use crate::*;
use std::borrow::Cow;

// ------ FontFamily ------

#[derive(Debug, Clone)]
pub enum FontFamily<'a> {
    Serif,
    SansSerif,
    Monospace,
    Custom(Cow<'a, str>),
}

impl<'a> FontFamily<'a> {
    pub fn new(family: impl IntoCowStr<'a>) -> Self {
        FontFamily::Custom(family.into_cow_str())
    }
}

impl<'a> IntoCowStr<'a> for FontFamily<'a> {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            FontFamily::Serif => "serif".into(),
            FontFamily::SansSerif => "sans-serif".into(),
            FontFamily::Monospace => "monospace".into(),
            FontFamily::Custom(family) => ["\"", family.as_ref(), "\""].concat().into(),
        }
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        unimplemented!()
    }
}
