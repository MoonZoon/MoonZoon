use crate::*;

// ------ Rgba ------

pub use cssparser_color::RgbaLegacy as Rgba;

impl IntoColor for Rgba {
    fn into_color(self) -> Color {
        Color::Rgba(self)
    }
}

// ------ Oklch ------

pub use cssparser_color::Oklch;

impl IntoColor for Oklch {
    fn into_color(self) -> Color {
        Color::Oklch(self)
    }
}

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

// ------ Hsl ------

pub use cssparser_color::Hsl;

impl IntoColor for Hsl {
    fn into_color(self) -> Color {
        Color::Hsl(self)
    }
}

// ------ Hwb ------

pub use cssparser_color::Hwb;

impl IntoColor for Hwb {
    fn into_color(self) -> Color {
        Color::Hwb(self)
    }
}

// ------ Lab ------

pub use cssparser_color::Lab;

impl IntoColor for Lab {
    fn into_color(self) -> Color {
        Color::Lab(self)
    }
}

// ------ Lch ------

pub use cssparser_color::Lch;

impl IntoColor for Lch {
    fn into_color(self) -> Color {
        Color::Lch(self)
    }
}

// ------ Oklab ------

pub use cssparser_color::Oklab;

impl IntoColor for Oklab {
    fn into_color(self) -> Color {
        Color::Oklab(self)
    }
}
