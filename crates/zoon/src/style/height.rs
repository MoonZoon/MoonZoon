use crate::*;

#[derive(Default)]
pub struct Height<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
    height_mode: HeightMode,
}

enum HeightMode {
    Exact,
    Fill,
}

// @TODO remove (in the entire codebase) once `derive_default_enum` is stable
// https://github.com/rust-lang/rust/issues/87517
impl Default for HeightMode {
    fn default() -> Self {
        Self::Exact
    }
}

impl<'a> Height<'a> {
    pub fn new(height: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", px(height));
        this.height_mode = HeightMode::Exact;
        this
    }

    pub fn with_signal(
        height: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let height = height.map(|height| height.into().map(px));
        this.dynamic_css_props
            .insert("height".into(), box_css_signal(height));
        this.height_mode = HeightMode::Exact;
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100%");
        this.height_mode = HeightMode::Fill;
        this
    }

    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100vh");
        this.height_mode = HeightMode::Exact;
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
        let height_mode_class = match self.height_mode {
            HeightMode::Exact => "exact_height",
            HeightMode::Fill => "fill_height",
        };
        raw_el = raw_el.class(&height_mode_class);

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
