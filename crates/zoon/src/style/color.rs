use crate::*;
use std::borrow::Cow;

// ------ Color ------

pub trait Color<'a>: IntoCowStr<'a> {}

// ------ HSLuv ------

pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> HSLuv {
    hsla(h, s, l, 100)
}

pub fn hsla(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>, a: impl Into<f64>) -> HSLuv {
    HSLuv {
        h: h.into().min(360.),
        s: s.into().min(100.),
        l: l.into().min(100.),
        a: a.into().min(100.),
    }
}

/// https://www.hsluv.org/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLuv {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

impl Color<'_> for HSLuv {}

impl<'a> IntoCowStr<'a> for HSLuv {
    fn into_cow_str(self) -> Cow<'a, str> {
        let (r, g, b) = hsluv::hsluv_to_rgb((self.h, self.s, self.l));
        crate::format!("rgba({r}% {g}% {b}% / {a}%)", r=r*100., g=g*100., b=b*100., a=self.a).into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}

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
