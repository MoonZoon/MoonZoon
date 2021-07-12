use crate::*;
use std::borrow::Cow;

// ------ Color ------

pub trait Color<'a>: IntoCowStr<'a> {}

// ------ NamedColor ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum NamedColor {
    Blue7,
    Green2,
    Green5,
    Gray5,
    Gray8,
    Gray10,
    Red2,
    Red5,
}

impl Color<'_> for NamedColor {}

impl<'a> IntoCowStr<'a> for NamedColor {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            NamedColor::Blue7 => "cornflowerblue",
            NamedColor::Green2 => "darkgreen",
            NamedColor::Green5 => "green",
            NamedColor::Gray5 => "gray",
            NamedColor::Gray8 => "lightgray",
            NamedColor::Gray10 => "white",
            NamedColor::Red2 => "darkred",
            NamedColor::Red5 => "firebrick",
        }
        .into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
