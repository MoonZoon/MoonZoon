use crate::*;

#[derive(Default)]
pub struct Align<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Align<'a> {
    // @TODO make it work in other elements (not only El, Column) / rename / more typed CSS API?
    pub fn center_x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("align-self", "center".into());
        this
    }

    // @TODO make it work in other elements (not only Row) / rename / more typed CSS API?
    pub fn left() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("align-self", "flex-start".into());
        this
    }

    // @TODO make it work in other elements (not only Row) / rename / more typed CSS API?
    pub fn right() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("align-self", "flex-end".into());
        this
    }
}

impl<'a> Style<'a> for Align<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        let Self { 
            static_css_props, 
            dynamic_css_props 
        } = self;
        CssPropsContainer {
            static_css_props,
            dynamic_css_props,
            task_handles: Vec::new()
        }
    }
}
