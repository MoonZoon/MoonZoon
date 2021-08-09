use crate::*;
use std::borrow::Cow;

// ------ FontWeight ------

pub trait FontWeight<'a>: IntoCowStr<'a> {}

// ------ NamedWeight ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum NamedWeight {
    Heavy,
    ExtraBold,
    Bold,
    SemiBold,
    Medium,
    Regular,
    Light,
    ExtraLight,
    Hairline,
}

impl FontWeight<'_> for NamedWeight {}

impl<'a> IntoCowStr<'a> for NamedWeight {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            NamedWeight::Heavy => "900",
            NamedWeight::ExtraBold => "800",
            NamedWeight::Bold => "700",
            NamedWeight::SemiBold => "600",
            NamedWeight::Medium => "500",
            NamedWeight::Regular => "400",
            NamedWeight::Light => "300",
            NamedWeight::ExtraLight => "200",
            NamedWeight::Hairline => "100",
        }
        .into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
