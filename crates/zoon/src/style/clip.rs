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
    ///         .s(Width::new(50))
    ///         .s(Height::new(50))
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
    ///         .s(Width::new(50))
    ///         .s(Height::new(50))
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
    ///         .s(Width::new(50))
    ///         .s(Height::new(50))
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
