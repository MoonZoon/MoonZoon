use crate::*;
use std::borrow::Cow;

/// Define transformation styling to update the shape or position of an element.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>.
#[derive(Default)]
pub struct Transform {
    /// Vector to chain transformations.
    transformations: Vec<Cow<'static, str>>,
    self_signal: Option<Box<dyn Signal<Item = Option<Self>> + Unpin>>,
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

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
        let transform = transform.map(|transform| transform.into());
        this.self_signal = Some(Box::new(transform));
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
        self.transformations.push(Cow::from(crate::format!(
            "translateY(-{}px)",
            distance.into()
        )));
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
        self.transformations.push(Cow::from(crate::format!(
            "translateY({}px)",
            distance.into()
        )));
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
        self.transformations.push(Cow::from(crate::format!(
            "translateX(-{}px)",
            distance.into()
        )));
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
        self.transformations.push(Cow::from(crate::format!(
            "translateX({}px)",
            distance.into()
        )));
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
            .push(Cow::from(crate::format!("rotateZ({}deg)", degrees.into())));
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
        self.transformations.push(Cow::from(crate::format!(
            "scale({})",
            percent.into() / 100.
        )));
        self
    }

    pub fn flip_horizontal(mut self) -> Self {
        self.transformations.push(Cow::from("rotateY(180deg)"));
        self
    }

    pub fn flip_vertical(mut self) -> Self {
        self.transformations.push(Cow::from("rotateX(180deg)"));
        self
    }
}

fn transformations_into_value(transformations: Vec<Cow<'static, str>>) -> Cow<'static, str> {
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

impl<'a> Style<'a> for Transform {
    fn merge_with_group(self, group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            transformations,
            self_signal,
        } = self;

        if let Some(self_signal) = self_signal {
            group.style_signal(
                "transform",
                self_signal.map(|transform| {
                    transform.map(|transform| transformations_into_value(transform.transformations))
                }),
            )
        } else {
            group.style("transform", transformations_into_value(transformations))
        }
    }
}
