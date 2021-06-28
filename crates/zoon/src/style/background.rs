use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps, box_css_signal};

#[derive(Default)]
pub struct Background<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Background<'a> {
    pub fn color(mut self, color: impl Color<'a>) -> Self {
        if let Some(color) = color.into_option_cow_str() {
            self.static_css_props.insert("color", color);
        }
        self
    }

    pub fn color_signal(mut self, color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static) -> Self {
        self.dynamic_css_props.insert("background-color", box_css_signal(color));
        self
    }
}

impl<'a> Style<'a> for Background<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
