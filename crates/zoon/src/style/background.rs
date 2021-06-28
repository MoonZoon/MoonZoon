use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps};

#[derive(Default)]
pub struct Background<'a> {
    _static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps<'static>,
}

impl<'a> Background<'a> {
    pub fn color_signal(mut self, color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static) -> Self {
        self.dynamic_css_props.insert("background-color", Box::new(color.map(|color| {
            Box::new(color) as Box<dyn IntoOptionCowStr<'static>>
        })));
        self
    }
}

impl Style for Background<'_> {
    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        raw_el
    }
}
