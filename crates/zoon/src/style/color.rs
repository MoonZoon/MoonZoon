use crate::*;
use std::borrow::Cow;

// ------ Color ------

pub trait Color<'a>: IntoCowStr<'a> {}

// ------ NamedColor ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum NamedColor {
    Green2,
    Green5,
    Gray8,
    Gray10,
}

impl Color<'_> for NamedColor {}

impl<'a> IntoCowStr<'a> for NamedColor {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            NamedColor::Green2 => "darkgreen",
            NamedColor::Green5 => "green",
            NamedColor::Gray8 => "lightgray",
            NamedColor::Gray10 => "white",
        }
        .into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
