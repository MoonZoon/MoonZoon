use crate::*;

#[derive(Default)]
pub struct Padding<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Padding<'a> {
    pub fn all(padding: u32) -> Self {
        Self::default().x(padding).y(padding)
    }

    pub fn x(self, x: u32) -> Self {
        self.left(x).right(x)
    }

    pub fn y(self, y: u32) -> Self {
        self.top(y).bottom(y)
    }

    pub fn top(mut self, top: u32) -> Self {
        self.static_css_props.insert("padding-top", px(top));
        self
    }

    pub fn right(mut self, right: u32) -> Self {
        self.static_css_props.insert("padding-right", px(right));
        self
    }

    pub fn bottom(mut self, bottom: u32) -> Self {
        self.static_css_props.insert("padding-bottom", px(bottom));
        self
    }

    pub fn left(mut self, left: u32) -> Self {
        self.static_css_props.insert("padding-left", px(left));
        self
    }
}

impl<'a> Style<'a> for Padding<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, css_prop_value) in self.static_css_props {
                style_group = if css_prop_value.important {
                    style_group.style(name, css_prop_value.value)
                } else {
                    style_group.style_important(name, css_prop_value.value)
                };
            }
            return (raw_el, Some(style_group));
        }
        for (name, css_prop_value) in self.static_css_props {
            raw_el = if css_prop_value.important {
                raw_el.style_important(name, &css_prop_value.value)
            } else {
                raw_el.style(name, &css_prop_value.value)
            };
        }
        (raw_el, None)
    }
}
