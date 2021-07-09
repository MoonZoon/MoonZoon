use crate::{format, *};
use std::borrow::Cow;
use std::collections::BTreeMap;

mod background;
mod color;
mod font;
mod padding;
mod spacing;
mod width;
mod height;

pub use background::Background;
pub use color::{Color, NamedColor};
pub use font::Font;
pub use padding::Padding;
pub use spacing::Spacing;
pub use width::Width;
pub use height::Height;

type StaticCSSProps<'a> = BTreeMap<&'a str, Cow<'a, str>>;
type DynamicCSSProps = BTreeMap<&'static str, BoxedCssSignal>;

type BoxedCssSignal = Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>;

fn box_css_signal(
    signal: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
) -> BoxedCssSignal {
    Box::new(signal.map(|value| Box::new(value) as Box<dyn IntoOptionCowStr<'static>>))
}

fn px<'a>(px: u32) -> Cow<'a, str> {
    format!("{}px", px).into()
}

// ------ Style ------

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }

    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps);

    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        let (static_css_props, dynamic_css_props) = self.into_css_props();
        for (name, value) in static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        raw_el
    }
}
