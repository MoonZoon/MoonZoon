use crate::*;

/// Style an element with scrollbars.
#[derive(Default)]
pub struct Scrollbars<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Scrollbars<'a> {
    /// Add horizontal and vertical scrollbars if needed.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .content(
    ///         "Lorem ipsum dolor sit amet,
    /// consectetur adipiscing elit. Donec placerat lacus in commodo molestie.",
    ///     )
    ///     .s(Scrollbars::both())
    ///     .s(Height::new(100))
    ///     .s(Width::new(50));
    /// ```
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow", "auto");
        this
    }

    /// Add only the horizontal scrollbar.
    /// More information available at <https://css-tricks.com/popping-hidden-overflow/>
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .content(
    ///         "Lorem ipsum dolor sit amet,
    /// consectetur adipiscing elit. Donec placerat lacus in commodo molestie.",
    ///     )
    ///     .s(Scrollbars::x_and_clip_y())
    ///     .s(Height::new(100))
    ///     .s(Width::new(50));
    /// ```
    pub fn x_and_clip_y() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "auto");
        this.static_css_props.insert("overflow-y", "hidden");
        this
    }
    /// Add only the vertical scrollbar.
    /// More information available at <https://css-tricks.com/popping-hidden-overflow/>
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .content(
    ///         "Lorem ipsum dolor sit amet,
    /// consectetur adipiscing elit. Donec placerat lacus in commodo molestie.",
    ///     )
    ///     .s(Scrollbars::y_and_clip_x())
    ///     .s(Height::new(100))
    ///     .s(Width::new(50));
    /// ```
    pub fn y_and_clip_x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-y", "auto");
        this.static_css_props.insert("overflow-x", "hidden");
        this
    }
}

impl<'a> Style<'a> for Scrollbars<'a> {
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
            return (raw_el, Some(style_group));
        }
        for (name, css_prop_value) in self.static_css_props {
            raw_el = if css_prop_value.important {
                raw_el.style_important(name, &css_prop_value.value)
            } else {
                raw_el.style(name, &css_prop_value.value)
            };
        }
        (raw_el, None)
    }
}
