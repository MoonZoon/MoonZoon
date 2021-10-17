use crate::*;
use std::borrow::Cow;

// ------ HSLuv ------

pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> HSLuv {
    hsla(h, s, l, 100)
}

pub fn hsla(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>, a: impl Into<f64>) -> HSLuv {
    HSLuv {
        h: h.into().clamp(0., 360.),
        s: s.into().clamp(0., 100.),
        l: l.into().clamp(0., 100.),
        a: a.into().clamp(0., 100.),
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

impl<'a> IntoCowStr<'a> for HSLuv {
    fn into_cow_str(self) -> Cow<'a, str> {
        let (r, g, b) = hsluv::hsluv_to_rgb((self.h, self.s, self.l));
        crate::format!(
            "rgba({r}% {g}% {b}% / {a}%)",
            r = r * 100.,
            g = g * 100.,
            b = b * 100.,
            a = self.a
        )
        .into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
