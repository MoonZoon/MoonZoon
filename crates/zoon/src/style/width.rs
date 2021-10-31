use crate::*;

#[derive(Default)]
pub struct Width<'a> {
    static_css_props: StaticCSSProps<'a>,
    static_css_classes: StaticCssClasses<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Width<'a> {
    pub fn new(width: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", px(width));
        this.static_css_classes.insert("exact_width".into());
        this.static_css_classes.remove("fill_width".into());
        this
    }

    pub fn with_signal(
        width: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let width = width.map(|width| width.into().map(px));
        this.dynamic_css_props.insert("width".into(), box_css_signal(width));
        this.static_css_classes.insert("exact_width".into());
        this.static_css_classes.remove("fill_width".into());
        this
    }

    pub fn zeros(zeros: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", ch(zeros));
        this.static_css_classes.insert("exact_width".into());
        this.static_css_classes.remove("fill_width".into());
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", "100%");
        this.static_css_classes.insert("fill_width".into());
        this.static_css_classes.remove("exact_width".into());
        this
    }

    pub fn min(mut self, width: u32) -> Self {
        self.static_css_props.insert("min-width", px(width));
        self
    }

    pub fn max(mut self, width: u32) -> Self {
        self.static_css_props.insert("max-width", px(width));
        self
    }
}

impl<'a> Style<'a> for Width<'a> {
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
