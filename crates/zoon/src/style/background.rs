use crate::*;

#[derive(Default)]
pub struct Background<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Background<'a> {
    pub fn color(mut self, color: impl Color<'a>) -> Self {
        self.static_css_props
            .insert("background-color", color.into_cow_str());
        self
    }

    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        self.dynamic_css_props
            .insert("background-color", box_css_signal(color));
        self
    }

    pub fn url(mut self, url: impl IntoCowStr<'a>) -> Self {
        let url = ["url(", &url.into_cow_str(), ")"].concat();
        self.static_css_props .insert("background-image", url.into());
        self
    }

    pub fn url_signal(
        mut self,
        url: impl Signal<Item = impl IntoCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        let url = url.map(|url| {
            ["url(", &url.into_cow_str(), ")"].concat()
        });
        self.dynamic_css_props
            .insert("background-image", box_css_signal(url));
        self
    }
}

impl<'a> Style<'a> for Background<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        let Self { 
            static_css_props, 
            dynamic_css_props 
        } = self;
        CssPropsContainer {
            static_css_props,
            dynamic_css_props,
            task_handles: Vec::new()
        }
    }
}
