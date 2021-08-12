use crate::*;
use std::borrow::Cow;
use std::collections::BTreeMap;

mod align;
pub use align::Align;

mod background;
pub use background::Background;

mod borders;
pub use borders::{Borders, Border};

mod color;
pub use color::{Color, NamedColor, HSLuv, hsl, hsla};

mod clip;
pub use clip::Clip;

mod font;
pub use font::{Font, NamedWeight, FontWeight, FontFamily};

mod height;
pub use height::Height;

mod padding;
pub use padding::Padding;

mod rounded_corners;
pub use rounded_corners::RoundedCorners;

mod scrollbars;
pub use scrollbars::Scrollbars;

mod shadows;
pub use shadows::{Shadows, Shadow};

mod spacing;
pub use spacing::Spacing;

mod transform;
pub use transform::Transform;

mod width;
pub use width::Width;

// --

pub type StaticCSSProps<'a> = BTreeMap<&'a str, Cow<'a, str>>;
pub type DynamicCSSProps = BTreeMap<&'static str, BoxedCssSignal>;

pub type BoxedCssSignal = Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>;

pub fn box_css_signal(
    signal: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
) -> BoxedCssSignal {
    Box::new(signal.map(|value| Box::new(value) as Box<dyn IntoOptionCowStr<'static>>))
}

pub fn px<'a>(px: impl IntoCowStr<'a>) -> Cow<'a, str> {
    [&px.into_cow_str(), "px"].concat().into()
}

// ------ Style ------

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }

    fn into_css_props_container(self) -> CssPropsContainer<'a>;

    fn update_raw_el_styles<T: RawEl>(self, mut raw_el: T) -> T {
        let CssPropsContainer { 
            static_css_props,
            dynamic_css_props,
            task_handles,
         } = self.into_css_props_container();

        for (name, value) in static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        if not(task_handles.is_empty()) {
            raw_el = raw_el.after_remove(move |_| drop(task_handles))
        }
        raw_el
    }
}

// ------ CssPropsContainer ------

pub struct CssPropsContainer<'a> {
    pub static_css_props: StaticCSSProps<'a>,
    pub dynamic_css_props: DynamicCSSProps,
    pub task_handles: Vec<TaskHandle>,
}
