use crate::*;

#[derive(Default)]
pub struct RoundedCorners<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> RoundedCorners<'a> {
    pub fn all(self, radius: u32) -> Self {
        self.top_left(radius).top_right(radius).bottom_left(radius).bottom_right(radius)
    }

    pub fn top_left(mut self, radius: u32) -> Self {
        self.static_css_props.insert("border-top-left-radius", px(radius));
        self
    }

    pub fn top_right(mut self, radius: u32) -> Self {
        self.static_css_props.insert("border-top-right-radius", px(radius));
        self
    }

    pub fn bottom_left(mut self, radius: u32) -> Self {
        self.static_css_props.insert("border-bottom-left-radius", px(radius));
        self
    }

    pub fn bottom_right(mut self, radius: u32) -> Self {
        self.static_css_props.insert("border-bottom-right-radius", px(radius));
        self
    }
}

impl<'a> Style<'a> for RoundedCorners<'a> {
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
