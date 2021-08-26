use crate::*;

#[derive(Default)]
pub struct Width<'a> {
    static_css_props: StaticCSSProps<'a>,
    static_css_classes: StaticCssClasses<'a>,
}

impl<'a> Width<'a> {
    pub fn new(width: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", px(width));
        this.static_css_classes.insert("exact_width".into());
        this.static_css_classes.remove("fill_width".into());
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", "100%".into());
        this.static_css_classes.insert("fill_width".into());
        this.static_css_classes.remove("exact_width".into());
        this
    }

    pub fn min(mut self, width: u32) -> Self {
        self.static_css_props.insert("min-width", px(width));
        self
    }

    pub fn max(mut self, width: u32) -> Self {
        self.static_css_props.insert("max-width", px(width));
        self
    }
}

impl<'a> Style<'a> for Width<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default()
            .static_css_props(self.static_css_props)
            .static_css_classes(self.static_css_classes)
    }
}
