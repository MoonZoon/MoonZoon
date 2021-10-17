use hsluv;

/// https://www.hsluv.org/

// ------ hsl ------

pub fn hsl(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>) -> HSLuv {
    hsla(h, s, l, 100)
}

// ------ hsla ------

pub fn hsla(h: impl Into<f64>, s: impl Into<f64>, l: impl Into<f64>, a: impl Into<f64>) -> HSLuv {
    HSLuv {
        h: h.into().clamp(0., 360.),
        s: s.into().clamp(0., 100.),
        l: l.into().clamp(0., 100.),
        a: a.into().clamp(0., 100.),
    }
}

// ------ HSLuv ------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HSLuv {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

impl HSLuv {
    pub const fn new_unchecked(h: f64, s: f64, l: f64, a: f64) -> Self {
        HSLuv { h, s, l, a}
    }

    pub fn to_rgb(&self) -> (f64, f64, f64) {
        hsluv::hsluv_to_rgb((self.h, self.s, self.l))
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
}
