use rust_hsluv::hsluv_to_rgb;
use palette::{WithAlpha, IntoColor, convert::FromColorUnclamped};
use std::fmt;

#[cfg(feature = "hsluv_macro")]
pub use hsluv_macro::hsluv;

/// https://www.hsluv.org/

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromColorUnclamped)]
pub struct HSLuv {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

impl WithAlpha<f32> for HSLuv {
    type Color = Self;
    type WithAlpha = Self;

    fn with_alpha(self, alpha: f32) -> Self::WithAlpha {
        self.set_a(alpha)
    }

    fn without_alpha(self) -> Self::Color {
        self.set_a(0)
    }

    fn split(self) -> (Self::Color, f32) {
        (self, self.a as f32)
    }
}

impl FromColorUnclamped<palette::Xyz> for HSLuv {
    fn from_color_unclamped(color: palette::Xyz) -> HSLuv {
        let hsluva: palette::Hsluva = color.into_color();
        Self {
            h: hsluva.hue.into_inner().into(),
            s: hsluva.saturation.into(),
            l: hsluva.l.into(),
            a: hsluva.alpha.into(),
        }
    }
}

impl FromColorUnclamped<HSLuv> for palette::Xyz {
    fn from_color_unclamped(color: HSLuv) -> palette::Xyz {
        let hsluva = palette::Hsluva::new(color.h as f32, color.s as f32, color.l as f32, color.a as f32);
        hsluva.into_color()
    }
}

impl fmt::Display for HSLuv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.to_rgb();
        let (r, g, b, a) = (r * 100., g * 100., b * 100., self.a);
        write!(f, "rgba({r}% {g}% {b}% / {a}%)")
    }
}

impl HSLuv {
    pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> Self {
        Self::hsla(h, s, l, 100)
    }

    pub fn hsla(
        h: impl Into<f64>,
        s: impl Into<f64>,
        l: impl Into<f64>,
        a: impl Into<f64>,
    ) -> Self {
        Self {
            h: h.into().clamp(0., 360.),
            s: s.into().clamp(0., 100.),
            l: l.into().clamp(0., 100.),
            a: a.into().clamp(0., 100.),
        }
    }

    pub const fn new_unchecked(h: f64, s: f64, l: f64, a: f64) -> Self {
        HSLuv { h, s, l, a }
    }

    pub fn to_rgb(&self) -> (f64, f64, f64) {
        hsluv_to_rgb((self.h, self.s, self.l))
    }

    // -- setters --

    pub fn set_h(mut self, h: impl Into<f64>) -> Self {
        self.h = h.into().clamp(0., 360.);
        self
    }

    pub fn set_s(mut self, s: impl Into<f64>) -> Self {
        self.s = s.into().clamp(0., 100.);
        self
    }

    pub fn set_l(mut self, l: impl Into<f64>) -> Self {
        self.l = l.into().clamp(0., 100.);
        self
    }

    pub fn set_a(mut self, a: impl Into<f64>) -> Self {
        self.a = a.into().clamp(0., 100.);
        self
    }

    // -- getters --

    pub fn h(&self) -> f64 {
        self.h
    }

    pub fn s(&self) -> f64 {
        self.s
    }

    pub fn l(&self) -> f64 {
        self.l
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    // -- updaters --

    pub fn update_h(self, h: impl FnOnce(f64) -> f64) -> Self {
        self.set_h(h(self.h))
    }

    pub fn update_s(self, s: impl FnOnce(f64) -> f64) -> Self {
        self.set_s(s(self.s))
    }

    pub fn update_l(self, l: impl FnOnce(f64) -> f64) -> Self {
        self.set_l(l(self.l))
    }

    pub fn update_a(self, a: impl FnOnce(f64) -> f64) -> Self {
        self.set_a(a(self.a))
    }
}
