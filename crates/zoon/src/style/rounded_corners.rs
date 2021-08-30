use crate::*;

#[derive(Default)]
pub struct RoundedCorners<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> RoundedCorners<'a> {
    pub fn all(radius: u32) -> Self {
        Self::default()
            .top(radius)
            .bottom(radius)
    }

    pub fn all_fully() -> Self {
        Self::default()
            .top_fully()
            .bottom_fully()
    }

    pub fn top(self, radius: u32) -> Self {
        self.top_left(radius).top_right(radius)
    }

    pub fn top_fully(self) -> Self {
        self.top_left_fully().top_right_fully()
    }

    pub fn bottom(self, radius: u32) -> Self {
        self.bottom_left(radius).bottom_right(radius)
    }

    pub fn bottom_fully(self) -> Self {
        self.bottom_left_fully().bottom_right_fully()
    }

    pub fn left(self, radius: u32) -> Self {
        self.top_left(radius).bottom_left(radius)
    }

    pub fn left_fully(self) -> Self {
        self.top_left_fully().bottom_left_fully()
    }

    pub fn right(self, radius: u32) -> Self {
        self.top_right(radius).bottom_right(radius)
    }

    pub fn right_fully(self) -> Self {
        self.top_right_fully().bottom_right_fully()
    }

    pub fn top_left(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-top-left-radius", px(radius));
        self
    }

    pub fn top_left_fully(self) -> Self {
        self.top_left(9999)
    }

    pub fn top_right(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-top-right-radius", px(radius));
        self
    }

    pub fn top_right_fully(self) -> Self {
        self.top_right(9999)
    }

    pub fn bottom_left(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-bottom-left-radius", px(radius));
        self
    }

    pub fn bottom_left_fully(self) -> Self {
        self.bottom_left(9999)
    }

    pub fn bottom_right(mut self, radius: u32) -> Self {
        self.static_css_props
            .insert("border-bottom-right-radius", px(radius));
        self
    }

    pub fn bottom_right_fully(self) -> Self {
        self.bottom_right(9999)
    }
}

impl<'a> Style<'a> for RoundedCorners<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_css_props(self.static_css_props)
    }
}
