use crate::*;
use std::{borrow::Cow, mem};

/// Define transformation styling to update the shape or position of an element.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>.
#[derive(Default)]
pub struct Transform {
    /// Vector to chain transformations.
    transformations: Vec<String>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl Transform {
    /// Apply transformations depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::with_signal(hover_signal.map_bool(
    ///             || Transform::new().rotate(45),
    ///             || Transform::new(),
    ///         )))
    ///         .on_hovered_change(move |hover| hovered.set(hover))
    ///         .label("Hover me"),
    /// );
    /// ```
    pub fn with_signal(
        transform: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let transform = transform.map(|transform| {
            transform
                .into()
                .map(|mut transform| transform.transformations_into_value())
        });
        this.dynamic_css_props
            .insert("transform".into(), box_css_signal(transform));
        this
    }

    /// Apply an upward translation to the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().move_up(50))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn move_up(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateY(-{}px)", distance.into()));
        self
    }

    /// Apply a downward translation to the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().move_down(50))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn move_down(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateY({}px)", distance.into()));
        self
    }

    /// Apply a leftward translation to the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().move_left(50))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn move_left(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateX(-{}px)", distance.into()));
        self
    }

    /// Apply a rightward translation to the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().move_right(50))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn move_right(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateX({}px)", distance.into()));
        self
    }

    /// Apply a rotation to the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().rotate(45))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn rotate(mut self, degrees: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("rotateZ({}deg)", degrees.into()));
        self
    }

    /// Apply scaling in `percentage` to the element.
    ///
    /// You can increase the size of the element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().scale(300))
    ///         .label("Click me"),
    /// );
    /// ```
    /// You can also decrease it.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let page = El::new().s(Height::screen()).child(
    ///     Button::new()
    ///         .s(Align::center())
    ///         .s(Transform::new().scale(50))
    ///         .label("Click me"),
    /// );
    /// ```
    pub fn scale(mut self, percent: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("scale({})", percent.into() / 100.));
        self
    }

    fn transformations_into_value(&mut self) -> Cow<'static, str> {
        let transformations = mem::take(&mut self.transformations);
        if transformations.is_empty() {
            return "none".into();
        }
        transformations
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    }
}

impl<'a> Style<'a> for Transform {
    fn apply_to_raw_el<E: RawEl>(
        mut self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        let mut static_css_props = StaticCSSProps::default();

        if self.dynamic_css_props.is_empty() {
            static_css_props.insert("transform", self.transformations_into_value());
        }

        if let Some(mut style_group) = style_group {
            for (name, css_prop_value) in static_css_props {
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
        for (name, css_prop_value) in static_css_props {
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
