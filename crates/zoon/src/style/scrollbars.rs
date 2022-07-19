use crate::*;

/// Style an element with scrollbars.
pub struct Scrollbars<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    visible: bool,
}

impl<'a> Default for Scrollbars<'a> {
    fn default() -> Self {
        Self {
            static_css_props: StaticCSSProps::default(),
            visible: true,
        }
    }
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
    ///     .s(Height::exact(100))
    ///     .s(Width::exact(50));
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
    ///     .s(Height::exact(100))
    ///     .s(Width::exact(50));
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
    ///     .s(Height::exact(100))
    ///     .s(Width::exact(50));
    /// ```
    pub fn y_and_clip_x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-y", "auto");
        this.static_css_props.insert("overflow-x", "hidden");
        this
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<'a> Style<'a> for Scrollbars<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        let Self {
            static_css_props,
            visible,
        } = self;
        groups.update_first(|mut group| {
            group.static_css_props.extend(static_css_props);
            if not(visible) {
                group = group
                    .style_unchecked("overflow-style", "none")
                    .style_unchecked("scrollbar-width", "none")
            }
            group
        });
        if not(visible) {
            groups.update_with_selector("::-webkit-scrollbar", |group| {
                group.style_unchecked("display", "none")
            });
        }
    }
}
