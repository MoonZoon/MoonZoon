use crate::*;

#[derive(Default)]
pub struct Align<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Align<'a> {
    pub fn center_x(mut self) -> Self {
        self.static_css_props.insert("align-self", "center".into());
        self
    }
}

impl<'a> Style<'a> for Align<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
