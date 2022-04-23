use crate::*;

#[derive(Default)]
pub struct Visible<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Visible<'a> {
    pub fn new(visible: bool) -> Self {
        let mut this = Self::default();
        let value = if visible {
            "visible"
        } else {
            "hidden"
        };
        this.static_css_props.insert("visibility", value);
        this
    }

    pub fn with_signal(
        visible: impl Signal<Item = impl Into<Option<bool>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let visible = visible.map(|visible| visible.into().map(|visible| {
            if visible {
                "visible"
            } else {
                "hidden"
            }
        }));
        this.dynamic_css_props
            .insert("visibility".into(), box_css_signal(visible));
        this
    }
}

impl<'a> Style<'a> for Visible<'a> {
    fn merge_with_group(self, mut group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            static_css_props,
            dynamic_css_props,
        } = self;
        group.static_css_props.extend(static_css_props);
        group.dynamic_css_props.extend(dynamic_css_props);
        group
    }
}
