use crate::*;
use std::collections::BTreeMap;
pub struct Font {
    css_props: BTreeMap<&'static str, &'static str>
}

impl Font {
    pub fn new() -> Self {
        Self {
            css_props: BTreeMap::new()
        }
    }

    pub fn bold(mut self) -> Self {
        self.css_props.insert("font-weight", "bold");
        self
    }
}

impl Style for Font {
    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        for (name, value) in self.css_props {
            raw_el = raw_el.style(name, value);
        }
        raw_el
    }
}
