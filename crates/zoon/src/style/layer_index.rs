use crate::*;

#[derive(Default)]
pub struct LayerIndex<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> LayerIndex<'a> {
    // Google says we can't use `i32::MIN/MAX` on all browsers directly, don't know why
    const MAX_VALUE_OFFSET: i32 = 9;
    pub const MIN_VALUE: i32 = i32::MIN + Self::MAX_VALUE_OFFSET;
    pub const MAX_VALUE: i32 = i32::MAX - Self::MAX_VALUE_OFFSET;

    pub fn new(index: i32) -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("z-index", index.into_cow_str());
        this
    }

    pub fn with_signal(
        index: impl Signal<Item = impl Into<Option<i32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let index = index.map(|index| index.into());
        this.dynamic_css_props
            .insert("z-index".into(), box_css_signal(index));
        this
    }
}

impl<'a> Style<'a> for LayerIndex<'a> {
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
