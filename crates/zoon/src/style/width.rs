use crate::style::{px, DynamicCSSProps, StaticCSSProps};
use crate::*;

#[derive(Default)]
pub struct Width<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Width<'a> {
    pub fn new(width: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", px(width));
        this
    }
}

impl<'a> Style<'a> for Width<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
