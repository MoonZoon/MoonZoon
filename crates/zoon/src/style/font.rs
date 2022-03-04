use crate::*;
use std::borrow::Cow;

mod font_weight;
pub use font_weight::FontWeight;

mod font_family;
pub use font_family::FontFamily;

mod font_line;
pub use font_line::FontLine;

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

    pub fn weight_signal(
        mut self,
        weight: impl Signal<Item = impl Into<Option<FontWeight>>> + Unpin + 'static,
    ) -> Self {
        let weight = weight.map(|weight| weight.into().map(|weight| weight.number()));
        self.dynamic_css_props
            .insert("font-weight".into(), box_css_signal(weight));
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

    pub fn wrap_anywhere(mut self) -> Self {
        // @TODO replace with the line below once `overflow-wrap: anywhere` works on Safari
        // https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-wrap#browser_compatibility
        self.static_css_props.insert("word-break", "break-word");
        // self.static_css_props.insert("overflow-wrap", "anywhere");

        self.static_css_props.remove("white-space");
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
        self.static_css_props.insert("font-family", font_family);
        self
    }

    pub fn line(mut self, line: FontLine<'a>) -> Self {
        self.static_css_props
            .extend(line.static_css_props.into_iter());
        self.dynamic_css_props
            .extend(line.dynamic_css_props.into_iter());
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
