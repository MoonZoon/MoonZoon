use crate::*;

#[derive(Default)]
pub struct Padding<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Padding<'a> {
    pub fn all(padding: u32) -> Self {
        Self::default().x(padding).y(padding)
    }

    pub fn all_signal(all: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let this = Self::default();
        let all = Broadcaster::new(all.map(|all| all.into()));
        this.x_signal(all.signal()).y_signal(all.signal())
    }

    pub fn x(self, x: u32) -> Self {
        self.left(x).right(x)
    }

    pub fn x_signal(self, x: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let x = Broadcaster::new(x.map(|x| x.into()));
        self.left_signal(x.signal()).right_signal(x.signal())
    }

    pub fn y(self, y: u32) -> Self {
        self.top(y).bottom(y)
    }

    pub fn y_signal(self, y: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let y = Broadcaster::new(y.map(|y| y.into()));
        self.top_signal(y.signal()).bottom_signal(y.signal())
    }

    pub fn top(mut self, top: u32) -> Self {
        self.static_css_props.insert("padding-top", px(top));
        self
    }

    pub fn top_signal(
        mut self,
        top: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let top = top.map(|top| top.into().map(px));
        self.dynamic_css_props
            .insert("padding-top".into(), box_css_signal(top));
        self
    }

    pub fn right(mut self, right: u32) -> Self {
        self.static_css_props.insert("padding-right", px(right));
        self
    }

    pub fn right_signal(
        mut self,
        right: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let right = right.map(|right| right.into().map(px));
        self.dynamic_css_props
            .insert("padding-right".into(), box_css_signal(right));
        self
    }

    pub fn bottom(mut self, bottom: u32) -> Self {
        self.static_css_props.insert("padding-bottom", px(bottom));
        self
    }

    pub fn bottom_signal(
        mut self,
        bottom: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let bottom = bottom.map(|bottom| bottom.into().map(px));
        self.dynamic_css_props
            .insert("padding-bottom".into(), box_css_signal(bottom));
        self
    }

    pub fn left(mut self, left: u32) -> Self {
        self.static_css_props.insert("padding-left", px(left));
        self
    }

    pub fn left_signal(
        mut self,
        left: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let left = left.map(|left| left.into().map(px));
        self.dynamic_css_props
            .insert("padding-left".into(), box_css_signal(left));
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
            for (name, value) in self.dynamic_css_props {
                style_group = style_group.style_signal(name, value);
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
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}
