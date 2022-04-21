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
    fn merge_with_group(self, group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self { static_css_props } = self;
        group.static_css_props.extend(static_css_props);
        group
    }
}
