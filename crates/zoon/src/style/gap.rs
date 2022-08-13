use crate::*;

/// Define gap between elements.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>.
#[derive(Default)]
pub struct Gap<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Gap<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gap between child elements in pixels.
    /// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let grid = Row::new()
    ///     .s(Gap::both(10))
    ///     .item(Column::new().item("first column"))
    ///     .item(Column::new().item("second column"));
    /// ```
    pub fn both(gap: u32) -> Self {
        Self::default().x(gap).y(gap)
    }

    pub fn both_signal(gap: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let this = Self::default();
        let gap = Broadcaster::new(gap.map(|gap| gap.into()));
        this.x_signal(gap.signal()).y_signal(gap.signal())
    }

    pub fn x(mut self, gap: u32) -> Self {
        self.static_css_props.insert("column-gap", px(gap));
        self
    }

    pub fn x_signal(
        mut self,
        gap: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let gap = gap.map(|gap| gap.into().map(px));
        self.dynamic_css_props
            .insert("column-gap".into(), box_css_signal(gap));
        self
    }

    pub fn y(mut self, gap: u32) -> Self {
        self.static_css_props.insert("row-gap", px(gap));
        self
    }

    pub fn y_signal(
        mut self,
        gap: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let gap = gap.map(|gap| gap.into().map(px));
        self.dynamic_css_props
            .insert("row-gap".into(), box_css_signal(gap));
        self
    }
}

impl<'a> Style<'a> for Gap<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self {
                static_css_props,
                dynamic_css_props,
            } = self;
            group.static_css_props.extend(static_css_props);
            group.dynamic_css_props.extend(dynamic_css_props);
            group
        });
    }
}
