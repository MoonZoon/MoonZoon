use crate::*;

#[derive(Default)]
pub struct FontLine<'a> {
    pub(crate) static_css_props: StaticCSSProps<'a>,
    pub(crate) dynamic_css_props: DynamicCSSProps,
}

impl FontLine<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn underline(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-line", "underline");
        self
    }

    pub fn underline_signal(
        mut self,
        underline: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Self {
        let underline = underline.map_bool(|| "underline", || "none");
        self.dynamic_css_props
            .insert("text-decoration-line".into(), box_css_signal(underline));
        self
    }

    pub fn strike(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-line", "line-through");
        self
    }

    pub fn strike_signal(mut self, strike: impl Signal<Item = bool> + Unpin + 'static) -> Self {
        let strike = strike.map_bool(|| "line-through", || "none");
        self.dynamic_css_props
            .insert("text-decoration-line".into(), box_css_signal(strike));
        self
    }

    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.static_css_props
                .insert("text-decoration-color", color.into_cow_str());
        }
        self
    }

    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let color = color.map(|color| color.into().map(|color| color.into_cow_str()));
        self.dynamic_css_props
            .insert("text-decoration-color".into(), box_css_signal(color));
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.static_css_props
            .insert("text-decoration-thickness", px(width));
        self
    }

    pub fn width_signal(
        mut self,
        width: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let width = width.map(|width| width.into().map(|width| px(width)));
        self.dynamic_css_props
            .insert("text-decoration-thickness".into(), box_css_signal(width));
        self
    }

    pub fn solid(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-style", "solid");
        self
    }

    pub fn double(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-style", "double");
        self
    }

    pub fn dotted(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-style", "dotted");
        self
    }

    pub fn dashed(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-style", "dashed");
        self
    }

    pub fn wavy(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration-style", "wavy");
        self
    }
}
