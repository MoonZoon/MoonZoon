use crate::*;

/// Define space between elements.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>.
#[derive(Default)]
pub struct Spacing<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Spacing<'a> {
    /// Space between child elements in pixels.
    /// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let grid = Row::new()
    ///     .s(Spacing::new(10))
    ///     .item(Column::new().item("first column"))
    ///     .item(Column::new().item("second column"));
    /// ```
    pub fn new(spacing: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("gap", px(spacing));
        this
    }
}

impl<'a> Style<'a> for Spacing<'a> {
    fn merge_with_group(self, group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self { static_css_props} = self;
        group.static_css_props.extend(static_css_props);
        group
    }
}
