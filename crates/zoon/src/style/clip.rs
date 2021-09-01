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
    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E, style_group: Option<StyleGroup<'a>>) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, value) in self.static_css_props {
                style_group = style_group.style(name, value);
            }
            return (raw_el, Some(style_group))
        }
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        (raw_el, None)
    }
}
