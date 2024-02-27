use crate::*;

pub use cssparser_color::Oklch;

pub fn oklch() -> Oklch {
    Oklch::new(None, None, None, None)
}

pub trait OklchExt {
    fn l(self, l: impl Into<f64>) -> Self;
    fn c(self, c: impl Into<f64>) -> Self;
    fn h(self, h: impl Into<f64>) -> Self;
    fn a(self, a: impl Into<f64>) -> Self;
}

impl OklchExt for Oklch {
    fn l(mut self, l: impl Into<f64>) -> Self {
        self.lightness = Some(l.into() as f32);
        self
    }
    fn c(mut self, c: impl Into<f64>) -> Self {
        self.chroma = Some(c.into() as f32);
        self
    }
    fn h(mut self, h: impl Into<f64>) -> Self {
        self.hue = Some(h.into() as f32);
        self
    }
    fn a(mut self, a: impl Into<f64>) -> Self {
        self.alpha = Some(a.into() as f32);
        self
    }
}
