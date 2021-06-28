use crate::*;
use std::borrow::Cow;

// ------ Color ------

pub trait Color<'a>: IntoOptionCowStr<'a> {}

// ------ NamedColor ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum NamedColor {
    Green2,
    Green5,
    Gray8,
    Gray10,
}

impl Color<'_> for NamedColor {}

impl<'a> IntoOptionCowStr<'a> for NamedColor {
    fn into_option_cow_str(self) -> Option<Cow<'a, str>> {
        let color = match self {
            NamedColor::Green2 => "darkgreen",
            NamedColor::Green5 => "green",
            NamedColor::Gray8 => "lightgray",
            NamedColor::Gray10 => "white",
        };
        Some(color.into())
    }

    fn take_into_option_cow_str(&mut self) -> Option<Cow<'a, str>> {
        self.into_option_cow_str()
    }
}
