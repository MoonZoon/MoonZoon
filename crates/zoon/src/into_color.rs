use crate::color::*;
use crate::*;

// ---- IntoColor ----

pub trait IntoColor {
    fn into_color(self) -> CssColor;

    fn into_color_string(self) -> String
    where
        Self: Sized,
    {
        self.into_color()
            .to_css_string(<_>::default())
            .unwrap_throw()
    }
}

impl<T: IntoCowStr<'static>> IntoColor for T {
    fn into_color(self) -> CssColor {
        let color_str = self.into_cow_str();
        let output = match CssColor::parse_string(&color_str) {
            Ok(color) => color,
            Err(error) => {
                crate::eprintln!("failed to parse CSS color '{color_str}': {error:#}");
                CssColor::default()
            }
        };
        output
    }
}

impl IntoColor for CssColor {
    fn into_color(self) -> CssColor {
        self
    }
}
impl IntoColor for OKLCH {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for RGBA {
    fn into_color(self) -> CssColor {
        self.into()
    }
}

impl IntoColor for A98 {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for HSL {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for HWB {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for LAB {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for LCH {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for OKLAB {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for P3 {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for ProPhoto {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for Rec2020 {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for SRGB {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for SRGBLinear {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for XYZd50 {
    fn into_color(self) -> CssColor {
        self.into()
    }
}
impl IntoColor for XYZd65 {
    fn into_color(self) -> CssColor {
        self.into()
    }
}

// ---- IntoOptionColor ----

pub trait IntoOptionColor {
    fn into_option_color(self) -> Option<CssColor>;

    fn into_option_color_string(self) -> Option<String>;
}

impl<T: IntoColor> IntoOptionColor for T {
    fn into_option_color(self) -> Option<CssColor> {
        Some(self.into_color())
    }

    fn into_option_color_string(self) -> Option<String> {
        Some(self.into_color_string())
    }
}

impl<T: IntoColor> IntoOptionColor for Option<T> {
    fn into_option_color(self) -> Option<CssColor> {
        self.map(|this| this.into_color())
    }

    fn into_option_color_string(self) -> Option<String> {
        self.map(|this| this.into_color_string())
    }
}
