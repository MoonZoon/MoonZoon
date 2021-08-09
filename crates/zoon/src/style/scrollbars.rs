use crate::style::{DynamicCSSProps, StaticCSSProps};
use crate::*;

#[derive(Default)]
pub struct Scrollbars<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Scrollbars<'a> {
    pub fn both(self, visible: bool) -> Self {
        self.x(visible).y(visible)
    }

    pub fn x(mut self, visible: bool) -> Self {
        let overflow = if visible { "scroll" } else { "hidden" };
        self.static_css_props.insert("overflow-x", overflow.into());
        self
    }

    pub fn y(mut self, visible: bool) -> Self {
        let overflow = if visible { "scroll" } else { "hidden" };
        self.static_css_props.insert("overflow-y", overflow.into());
        self
    }
}

impl<'a> Style<'a> for Scrollbars<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
