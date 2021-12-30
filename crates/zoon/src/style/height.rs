use crate::*;

#[derive(Default)]
pub struct Height<'a> {
    static_css_props: StaticCSSProps<'a>,
    static_css_classes: StaticCssClasses<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Height<'a> {
    pub fn new(height: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", px(height));
        this.static_css_classes.insert("exact_height".into());
        this.static_css_classes.remove("fill_height".into());
        this
    }

    pub fn with_signal(
        height: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let height = height.map(|height| height.into().map(px));
        this.dynamic_css_props
            .insert("height".into(), box_css_signal(height));
        this.static_css_classes.insert("exact_height".into());
        this.static_css_classes.remove("fill_height".into());
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100%");
        this.static_css_classes.insert("fill_height".into());
        this.static_css_classes.remove("exact_height".into());
        this
    }

    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100vh");
        this.static_css_classes.insert("exact_height".into());
        this.static_css_classes.remove("fill_height".into());
        this
    }

    pub fn min_screen(mut self) -> Self {
        self.static_css_props.insert("min-height", "100vh");
        self
    }

    pub fn max(mut self, height: u32) -> Self {
        self.static_css_props.insert("max-height", px(height));
        self
    }

    pub fn max_fill(mut self) -> Self {
        self.static_css_props.insert("max-height", "100%");
        self
    }
}

impl<'a> Style<'a> for Height<'a> {
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
            for class in self.static_css_classes {
                raw_el = raw_el.class(&class);
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
        for class in self.static_css_classes {
            raw_el = raw_el.class(&class);
        }
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}
