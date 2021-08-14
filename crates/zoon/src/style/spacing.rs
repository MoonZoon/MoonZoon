use crate::*;

#[derive(Default)]
pub struct Spacing<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Spacing<'a> {
    pub fn new(spacing: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("gap", px(spacing));
        this
    }
}

impl<'a> Style<'a> for Spacing<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_css_props(self.static_css_props)
    }
}
