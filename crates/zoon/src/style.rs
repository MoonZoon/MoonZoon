use crate::*;
use std::collections::BTreeMap;

mod background;
mod font;
mod color;

pub use background::Background;
pub use font::Font;
pub use color::{Color, NamedColor};

type StaticCSSProps<'a> = BTreeMap<&'a str, &'a str>;
type DynamicCSSProps<'a> = BTreeMap<&'a str, Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'a>>> + Unpin>>;

pub trait Style: Default {
    fn new() -> Self {
        Self::default()
    }

    fn update_raw_el_style<T: RawEl>(self, raw_el: T) -> T;
}




