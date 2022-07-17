use crate::*;

pub trait Styleable<'a>: UpdateRawEl + Sized {
    /// Add `Zoon` styling to the element.
    /// # Example
    ///  Here is how to use [Align] to `center` an element.
    /// ```no_run
    /// use zoon::*;
    ///
    /// let centered_element = El::new().s(Align::center()).child("centered element");
    /// ```
    ///
    /// # Example
    /// Here we customize the style with [Width], [Height], [Align] and [Font]
    /// and add a change of color for background when the user `hovers` the
    /// button
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    ///
    /// let button = Button::new()
    ///     .s(Align::center())
    ///     .s(Width::exact(140))
    ///     .s(Height::exact(140))
    ///     .s(Font::new().size(30).center())
    ///     .s(Background::new().color_signal(hover_signal.map_bool(|| GREEN_2, || GREEN_1)))
    ///     .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
    ///     .label("Hover me");
    ///
    /// let view = Column::new().s(Height::screen()).item(button);
    /// ```
    /// # Example
    /// It is possible to use more than one signal and apply many styling
    /// effects.
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let hovered = Mutable::new(false);
    ///
    /// let button = Button::new()
    ///     .s(Align::center())
    ///     .s(Width::exact(140))
    ///     .s(Height::exact(140))
    ///     .s(Font::new()
    ///         .size(30)
    ///         .center()
    ///         .color_signal(hovered.signal().map_bool(|| PINK_5, || PINK_7)))
    ///     .s(Background::new().color_signal(hovered.signal().map_bool(|| GREEN_7, || GREEN_2)))
    ///     .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
    ///     .label("Hover me");
    ///
    /// let view = Column::new().s(Height::screen()).item(button);
    /// ```
    fn s<S: Style<'a>>(self, style: impl Into<Option<S>>) -> Self {
        if let Some(style) = style.into() {
            return self.update_raw_el(|raw_el| {
                raw_el.style_group(style.merge_with_group(StyleGroup::default()))
            });
        }
        self
    }
}
