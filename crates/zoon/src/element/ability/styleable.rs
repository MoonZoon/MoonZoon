use crate::*;

pub trait Styleable<'a, T: RawEl>: UpdateRawEl<T> + Sized {
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
    /// let view = Column::new().s(Height::screen()).item(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Width::new(140))
    ///         .s(Height::new(140))
    ///         .s(Font::new().size(30).center())
    ///         .s(Background::new()
    ///             .color_signal(hover_signal.map_bool(|| GREEN_2, || GREEN_1)))
    ///         .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
    ///         .label("Hover me"),
    /// );
    /// ```
    /// # Example
    /// It is possible to use more than one signal and apply  many styling
    /// effects.
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let (hovered_2, hover_signal_2) = Mutable::new_and_signal(false);
    /// Column::new().s(Height::screen()).item(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Width::new(140))
    ///         .s(Height::new(140))
    ///         .s(Font::new()
    ///             .size(30)
    ///             .center()
    ///             .color_signal(hover_signal.map_bool(|| PINK_5, || PINK_7)))
    ///         .s(Background::new()
    ///             .color_signal(hover_signal_2.map_bool(|| GREEN_2, || GREEN_7)))
    ///         .on_hovered_change(move |is_hovered| {
    ///             hovered.set(is_hovered);
    ///             hovered_2.set(is_hovered)
    ///         })
    ///         .label("Hover me"),
    /// );
    /// ```
    fn s(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|raw_el| style.apply_to_raw_el(raw_el, None).0)
    }
}
