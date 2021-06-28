use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps};

#[derive(Default)]
pub struct Font<'a> {
    static_css_props: StaticCSSProps<'a>,
    _dynamic_css_props: DynamicCSSProps<'a>,
}

impl Font<'_> {
    pub fn bold(mut self) -> Self {
        self.static_css_props.insert("font-weight", "bold");
        self
    }
}

impl Style for Font<'_> {
    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, value);
        }
        raw_el
    }
}
