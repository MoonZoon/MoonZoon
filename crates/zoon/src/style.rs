use crate::*;
use std::collections::BTreeMap;

mod background;
mod font;
mod color;

pub use background::Background;
pub use font::Font;
pub use color::{Color, NamedColor};

type StaticCSSProps<'a> = BTreeMap<&'a str, &'a str>;
type DynamicCSSProps = BTreeMap<&'static str, Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>>;

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }
    
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps);

    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        let (static_css_props, dynamic_css_props) = self.into_css_props();
        for (name, value) in static_css_props {
            raw_el = raw_el.style(name, value);
        }
        for (name, value) in dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        raw_el
    }

}




