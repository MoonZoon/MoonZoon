use crate::*;

/// Styling to set the background for an element.
/// # Example
/// ```no_run
/// use zoon::*;
///
/// let element = El::new().s(Background::new().color(hsluv!(241.3, 100, 96.6)));
/// ```
#[derive(Default)]
pub struct Background<'a> {
    /// Css properties used by Zoon to style the element
    /// with Css.
    static_css_props: StaticCSSProps<'a>,
    /// Css properties that can be used to customize the background directly.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Background<'a> {
    /// Set a given color to the background with HSLuv.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let element = El::new().s(Background::new().color(hsluv!(241.3, 100, 96.6)));
    /// ```
    ///
    /// Set a given color to the background with predefined colors.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let element = El::new().s(Background::new().color(BLUE_0));
    /// ```
    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.static_css_props
                .insert("background-color", color.into_cow_str());
        }
        self
    }

    /// Set the color depending of the signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    ///
    /// let button = Button::new()
    ///     .s(Background::new().color_signal(hovered_signal.map_bool(|| BLUE_0, || BLUE_9)))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered));
    /// ```
    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let color = color.map(|color| color.into().map(|color| color.into_cow_str()));
        self.dynamic_css_props
            .insert("background-color".into(), box_css_signal(color));
        self
    }

    /// Can be used to set an image as background.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    /// let element = El::new().s(Background::new().url("/assets/images/stars.png"));
    /// ```
    pub fn url(mut self, url: impl IntoCowStr<'a>) -> Self {
        let url = ["url(", &url.into_cow_str(), ")"].concat();
        self.static_css_props.insert("background-image", url);
        self
    }

    /// Can set the url depending of the signal's state.
    ///
    /// Here the background changes depending of the click state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (traveling, travel_to_space) = Mutable::new_and_signal(false);
    ///
    /// let element = El::new()
    ///     .s(Background::new().url_signal(travel_to_space.map_bool(
    ///         || "/assets/images/stars.png",
    ///         || "/assets/images/launch_pad.png",
    ///     )))
    ///     .child("Travel to space")
    ///     .on_click(move || traveling.set(!traveling.take()));
    /// ```
    pub fn url_signal(
        mut self,
        url: impl Signal<Item = impl IntoCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        let url = url.map(|url| ["url(", &url.into_cow_str(), ")"].concat());
        self.dynamic_css_props
            .insert("background-image".into(), box_css_signal(url));
        self
    }
}

impl<'a> Style<'a> for Background<'a> {
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
