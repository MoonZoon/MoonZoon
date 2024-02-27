use crate::cssparser::{Parser, ParserInput};
use crate::*;

// ---- IntoColor ----

pub trait IntoColor {
    fn into_color(self) -> Color;

    fn into_color_string(self) -> String
    where
        Self: Sized,
    {
        self.into_color().to_css_string()
    }
}

impl<T: IntoCowStr<'static>> IntoColor for T {
    fn into_color(self) -> Color {
        let color_str = self.into_cow_str();
        let output = match Color::parse(&mut Parser::new(&mut ParserInput::new(&color_str))) {
            Ok(color) => color,
            Err(error) => {
                crate::eprintln!("failed to parse CSS color '{color_str}': {error:?}");
                oklch().into_color()
            }
        };
        output
    }
}

impl IntoColor for Color {
    fn into_color(self) -> Color {
        self
    }
}

// Note: Other `IntoColor` implementations in `color_space.rs`

// ---- IntoOptionColor ----

pub trait IntoOptionColor {
    fn into_option_color(self) -> Option<Color>;

    fn into_option_color_string(self) -> Option<String>;
}

impl<T: IntoColor> IntoOptionColor for T {
    fn into_option_color(self) -> Option<Color> {
        Some(self.into_color())
    }

    fn into_option_color_string(self) -> Option<String> {
        Some(self.into_color_string())
    }
}

impl<T: IntoColor> IntoOptionColor for Option<T> {
    fn into_option_color(self) -> Option<Color> {
        self.map(|this| this.into_color())
    }

    fn into_option_color_string(self) -> Option<String> {
        self.map(|this| this.into_color_string())
    }
}
