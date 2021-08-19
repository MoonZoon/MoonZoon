use crate::*;

#[derive(Default)]
pub struct Height<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Height<'a> {
    pub fn new(height: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", px(height));
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100%".into());
        this
    }

    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100vh".into());
        this
    }

    pub fn min_screen(mut self) -> Self {
        self.static_css_props.insert("min-height", "100vh".into());
        self
    }
}

impl<'a> Style<'a> for Height<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_css_props(self.static_css_props)
    }
}
