use crate::*;

#[derive(Default)]
pub struct Scrollbars<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Scrollbars<'a> {
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow", "auto".into());
        this
    }

    /// https://css-tricks.com/popping-hidden-overflow/
    pub fn x_and_clip_y() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "auto".into());
        this.static_css_props.insert("overflow-y", "hidden".into());
        this
    }

    /// https://css-tricks.com/popping-hidden-overflow/
    pub fn y_and_clip_x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-y", "auto".into());
        this.static_css_props.insert("overflow-x", "hidden".into());
        this
    }
}

impl<'a> Style<'a> for Scrollbars<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
