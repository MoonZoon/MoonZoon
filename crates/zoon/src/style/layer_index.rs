use crate::*;

/// Styling to define the value on the `z` axis for an element. It does
/// translate to z-index in css.
#[derive(Default)]
pub struct LayerIndex<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> LayerIndex<'a> {
    // Google says we can't use `i32::MIN/MAX` on all browsers directly, don't know
    // why
    const MAX_VALUE_OFFSET: i32 = 9;
    pub const MIN_VALUE: i32 = i32::MIN + Self::MAX_VALUE_OFFSET;
    pub const MAX_VALUE: i32 = i32::MAX - Self::MAX_VALUE_OFFSET;

    /// Set the layer index for an element.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let contained_items = Column::new()
    ///     .s(Background::new().color(GREEN_7))
    ///     .item(
    ///         Paragraph::new()
    ///             .s(LayerIndex::new(-1))
    ///             .content("Behind and trying to hide"),
    ///     )
    ///     .item(Paragraph::new().content("Front and you can see me"));
    /// ```
    pub fn new(index: i32) -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("z-index", index.into_cow_str());
        this
    }

    /// Set the layer index for an element depending on signal's state..
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let (is_hovered, hover_signal) = Mutable::new_and_signal(false);
    ///
    /// let container = Column::new()
    ///     .s(Background::new().color(GREEN_7))
    ///     .item(
    ///         Paragraph::new()
    ///             .s(LayerIndex::with_signal(hover_signal.map_false(|| -1)))
    ///             .content("Behind and trying to hide"),
    ///     )
    ///     .item(
    ///         Paragraph::new()
    ///             .content("Front and you can see me. Hover me so you can see a surprise.")
    ///             .on_hovered_change(move |hover| is_hovered.set(hover)),
    ///     );
    /// ```
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
