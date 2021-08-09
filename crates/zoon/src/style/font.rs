use crate::style::{box_css_signal, px, DynamicCSSProps, StaticCSSProps};
use crate::*;

#[derive(Default)]
pub struct Font<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Font<'a> {
    pub fn bold(mut self) -> Self {
        self.static_css_props.insert("font-weight", "bold".into());
        self
    }

    pub fn color(mut self, color: impl Color<'a>) -> Self {
        self.static_css_props.insert("color", color.into_cow_str());
        self
    }

    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        self.dynamic_css_props
            .insert("color", box_css_signal(color));
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.static_css_props.insert("font-size", px(size));
        self
    }

    pub fn underline(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration", "underline".into());
        self
    }

    pub fn underline_signal(
        mut self,
        underline: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Self {
        let underline = underline.map_bool(|| "underline", || "none");
        self.dynamic_css_props
            .insert("text-decoration", box_css_signal(underline));
        self
    }

    pub fn strike(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration", "line-through".into());
        self
    }

    pub fn strike_signal(
        mut self,
        strike: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Self {
        let strike = strike.map_bool(|| "line-through", || "none");
        self.dynamic_css_props
            .insert("text-decoration", box_css_signal(strike));
        self
    }

    pub fn center(mut self) -> Self {
        self.static_css_props
            .insert("text-align", "center".into());
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}
