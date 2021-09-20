use crate::*;

#[derive(Default)]
pub struct Scrollbars<'a> {
    static_css_props: StaticCSSProps<'a>,
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
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, value) in self.static_css_props {
                style_group = style_group.style(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        (raw_el, None)
    }
}
