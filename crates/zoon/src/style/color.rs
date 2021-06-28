use crate::*;
use std::borrow::Cow;

// ------ Color ------

pub trait Color<'a>: IntoCowStr<'a> {}

// ------ NamedColor ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum NamedColor {
    Green2,
    Green5,
}

impl Color<'_> for NamedColor {}

impl<'a> IntoCowStr<'a> for NamedColor {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            NamedColor::Green2 => {
                "darkgreen".into() 
            }
            NamedColor::Green5 => {
                "darkgreen".into()
            }
        }
    }
}
