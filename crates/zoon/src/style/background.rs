use crate::*;

pub struct Background {
}

impl Background {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn color_signal<'a>(self, color: impl Signal<Item = impl Color<'a>> + Unpin + 'static) -> Self {
        // self.css_props.insert("font-weight", "bold");
        self
    }
}

impl Style for Background {
    fn update_raw_el_style<T: RawEl>(self, raw_el: T) -> T {
        raw_el
    }
}
