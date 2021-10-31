use crate::*;
use std::borrow::Cow;

mod font_weight;
pub use font_weight::FontWeight;

mod font_family;
pub use font_family::FontFamily;

#[derive(Default)]
pub struct Font<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Font<'a> {
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.static_css_props
            .insert("font-weight", weight.number().into_cow_str());
        self
    }

    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.static_css_props.insert("color", color.into_cow_str());
        }
        self
    }

    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let color = color.map(|color| color.into().map(|color| color.into_cow_str()));
        self.dynamic_css_props
            .insert("color".into(), box_css_signal(color));
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.static_css_props.insert("font-size", px(size));
        self
    }

    pub fn size_signal(
        mut self,
        size: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let size = size.map(|size| size.into().map(px));
        self.dynamic_css_props
            .insert("font-size".into(), box_css_signal(size));
        self
    }

    pub fn line_height(mut self, line_height: u32) -> Self {
        self.static_css_props.insert("line-height", px(line_height));
        self
    }

    pub fn italic(mut self) -> Self {
        self.static_css_props.insert("font-style", "italic");
        self
    }

    pub fn no_wrap(mut self) -> Self {
        self.static_css_props.insert("white-space", "nowrap");
        self
    }

    pub fn underline(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration", "underline");
        self
    }

    pub fn underline_signal(
        mut self,
        underline: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Self {
        let underline = underline.map_bool(|| "underline", || "none");
        self.dynamic_css_props
            .insert("text-decoration".into(), box_css_signal(underline));
        self
    }

    pub fn strike(mut self) -> Self {
        self.static_css_props
            .insert("text-decoration", "line-through");
        self
    }

    pub fn strike_signal(mut self, strike: impl Signal<Item = bool> + Unpin + 'static) -> Self {
        let strike = strike.map_bool(|| "line-through", || "none");
        self.dynamic_css_props
            .insert("text-decoration".into(), box_css_signal(strike));
        self
    }

    pub fn center(mut self) -> Self {
        self.static_css_props.insert("text-align", "center");
        self
    }

    pub fn family(mut self, family: impl IntoIterator<Item = FontFamily<'a>>) -> Self {
        let font_family = family
            .into_iter()
            .map(|family| family.into_cow_str())
            .collect::<Cow<_>>()
            .join(", ");
        self.static_css_props
            .insert("font-family", font_family);
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
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
