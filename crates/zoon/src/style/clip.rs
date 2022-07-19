use crate::*;

/// Clip the element by its parent or a mask to show only a specific area.
#[derive(Default)]
pub struct Clip<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Clip<'a> {
    /// THe element gets vertically and horizontally clipped by its parent.
    /// # Example
    /// ```no_run
    ///
    /// use zoon::{*, named_color::* };
    ///
    /// let paragraph =   Paragraph::new()
    ///     .s(Font::new()).s(Clip::x())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    ///
    ///  let parent_element =  Column::new()
    ///         .s(Width::exact(50))
    ///         .s(Height::exact(50))
    ///         .s(Background::new().color(BLUE_9))
    ///         .item(paragraph);
    /// ```
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "hidden");
        this.static_css_props.insert("overflow-y", "hidden");
        this
    }

    /// The element gets clipped by its parent horizontally.
    /// # Example
    /// ```no_run
    ///
    /// use zoon::{*, named_color::* };
    ///
    /// let paragraph =   Paragraph::new()
    ///     .s(Font::new()).s(Clip::x())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    ///
    ///  let parent_element =  Column::new()
    ///         .s(Width::exact(50))
    ///         .s(Height::exact(50))
    ///         .s(Background::new().color(BLUE_9))
    ///         .item(paragraph);
    /// ```
    pub fn x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-x", "hidden");
        this
    }

    /// The element gets clipped by its parent vertically.
    /// # Example
    /// ```no_run
    ///
    /// use zoon::{*, named_color::* };
    ///
    /// let paragraph =   Paragraph::new()
    ///     .s(Font::new()).s(Clip::x())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    ///
    ///  let parent_element =  Column::new()
    ///         .s(Width::exact(50))
    ///         .s(Height::exact(50))
    ///         .s(Background::new().color(BLUE_9))
    ///         .item(paragraph);
    /// ```
    pub fn y() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("overflow-y", "hidden");
        this
    }
}

impl<'a> Style<'a> for Clip<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self { static_css_props } = self;
            group.static_css_props.extend(static_css_props);
            group
        });
    }
}
