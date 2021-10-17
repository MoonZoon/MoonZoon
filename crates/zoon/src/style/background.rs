use crate::*;

#[derive(Default)]
pub struct Background<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Background<'a> {
    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.static_css_props
                .insert("background-color", color.into_cow_str());
        }
        self
    }

    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let color = color.map(|color| color.into().map(|color| color.into_cow_str()));
        self.dynamic_css_props
            .insert("background-color".into(), box_css_signal(color));
        self
    }

    pub fn url(mut self, url: impl IntoCowStr<'a>) -> Self {
        let url = ["url(", &url.into_cow_str(), ")"].concat();
        self.static_css_props.insert("background-image", url.into());
        self
    }

    pub fn url_signal(
        mut self,
        url: impl Signal<Item = impl IntoCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        let url = url.map(|url| ["url(", &url.into_cow_str(), ")"].concat());
        self.dynamic_css_props
            .insert("background-image".into(), box_css_signal(url));
        self
    }
}

impl<'a> Style<'a> for Background<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, value) in self.static_css_props {
                style_group = style_group.style(name, value);
            }
            for (name, value) in self.dynamic_css_props {
                style_group = style_group.style_signal(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}
