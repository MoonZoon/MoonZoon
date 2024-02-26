use crate::cssparser::{Parser, ParserInput};
use crate::*;

// ---- IntoColor ----

pub trait IntoColor {
    fn into_color(self) -> CssColor;

    fn into_color_string(self) -> String
    where
        Self: Sized,
    {
        self.into_color().to_css_string()
    }
}

impl<T: IntoCowStr<'static>> IntoColor for T {
    fn into_color(self) -> CssColor {
        let color_str = self.into_cow_str();
        let output = match CssColor::parse(&mut Parser::new(&mut ParserInput::new(&color_str))) {
            Ok(color) => color,
            Err(error) => {
                crate::eprintln!("failed to parse CSS color '{color_str}': {error:?}");
                oklch().into_color()
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

// Note: Other `IntoColor` implementations in `color_space.rs`

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
