use crate::*;

#[derive(Default)]
pub struct Clip<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Clip<'a> {
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "hidden".into());
        this.static_css_props.insert("overflow-y", "hidden".into());
        this
    }

    pub fn x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "hidden".into());
        this
    }

    pub fn y() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-y", "hidden".into());
        this
    }
}

impl<'a> Style<'a> for Clip<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_css_props(self.static_css_props)
    }
}
