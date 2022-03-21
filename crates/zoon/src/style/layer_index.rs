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
    ///             .s(LayerIndex::with_signal(hover_signal.map_bool(|| 1, || -1)))
    ///             .content("Behind and trying to hide"),
    ///     )
    ///     .item(
    ///         Paragraph::new()
    ///             .content("Front and you can see me. Hover me so you can see a surprise")
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
