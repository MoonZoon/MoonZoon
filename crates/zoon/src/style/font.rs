use crate::*;
use crate::style::{StaticCSSProps, DynamicCSSProps};

#[derive(Default)]
pub struct Font<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl Font<'_> {
    pub fn bold(mut self) -> Self {
        self.static_css_props.insert("font-weight", "bold");
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
