use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps, box_css_signal, px};

#[derive(Default)]
pub struct Font<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Font<'a> {
    pub fn bold(mut self) -> Self {
        self.static_css_props.insert("font-weight", "bold".into());
        self
    }

    pub fn color(mut self, color: impl Color<'a>) -> Self {
        if let Some(color) = color.into_option_cow_str() {
            self.static_css_props.insert("color", color);
        }
        self
    }

    pub fn color_signal(mut self, color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static) -> Self {
        self.dynamic_css_props.insert("color", box_css_signal(color));
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.static_css_props.insert("font-size", px(size));
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
