use crate::*;

#[derive(Default)]
pub struct RoundedCorners<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> RoundedCorners<'a> {
    pub fn all(radius: u32) -> Self {
        Self::default()
            .top_left(radius)
            .top_right(radius)
            .bottom_left(radius)
            .bottom_right(radius)
    }

    pub fn top(self, radius: u32) -> Self {
        self.top_left(radius).top_right(radius)
    }

    pub fn bottom(self, radius: u32) -> Self {
        self.bottom_left(radius).bottom_right(radius)
    }

    pub fn left(self, radius: u32) -> Self {
        self.top_left(radius).bottom_left(radius)
    }

    pub fn right(self, radius: u32) -> Self {
        self.top_right(radius).bottom_right(radius)
    }

    pub fn top_left(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-top-left-radius", px(radius));
        self
    }

    pub fn top_right(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-top-right-radius", px(radius));
        self
    }

    pub fn bottom_left(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-bottom-left-radius", px(radius));
        self
    }

    pub fn bottom_right(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-bottom-right-radius", px(radius));
        self
    }
}

impl<'a> Style<'a> for RoundedCorners<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_css_props(self.static_css_props)
    }
}
