use crate::*;

#[derive(Default)]
pub struct Spacing<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Spacing<'a> {
    pub fn new(spacing: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("gap", px(spacing));
        this
    }
}

impl<'a> Style<'a> for Spacing<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
