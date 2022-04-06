use crate::*;
use std::borrow::Cow;

// ------ Shadows ------

/// Style to add shadows to an element.
#[derive(Default)]
pub struct Shadows<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Shadows<'a> {
    /// Add new shadows.
    /// The method accepts an iterator of [Shadow] so multiple effects can be
    /// used. More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new()
    ///         .color(RED_3)
    ///         .y(10)
    ///         .x(10)
    ///         .blur(5)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn new(shadows: impl IntoIterator<Item = Shadow>) -> Self {
        let shadows = shadows
            .into_iter()
            .map(|shadow| shadow.into_cow_str())
            .collect::<Cow<_>>()
            .join(", ");
        let mut this = Self::default();
        this.static_css_props.insert("box-shadow", shadows);
        this
    }
    /// Add new shadows depending of signal's state.
    /// The method accepts an iterator of [Shadow] so multiple effects can be
    /// used.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Shadows::with_signal(hover_signal.map_bool(
    ///         || [Shadow::new().color(RED_8).y(10).x(10)],
    ///         || [Shadow::new().color(RED_3).y(10).x(10).blur(5)],
    ///     )))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .on_hovered_change(move |h| hovered.set(h))
    ///     .label("Click me");
    /// ```
    pub fn with_signal(
        shadows: impl Signal<Item = impl IntoIterator<Item = Shadow>> + Unpin + 'static,
    ) -> Self {
        let shadows = shadows.map(|shadows| {
            shadows
                .into_iter()
                .map(|shadow| shadow.into_cow_str())
                .collect::<Cow<_>>()
                .join(", ")
        });
        let mut this = Self::default();
        this.dynamic_css_props
            .insert("box-shadow".into(), box_css_signal(shadows));
        this
    }
}

impl<'a> Style<'a> for Shadows<'a> {
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

// ------ Shadow ------

/// Single Shadow effect.
/// More information available at <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>.
#[derive(Default)]
pub struct Shadow {
    inner: bool,
    x: i32,
    y: i32,
    spread: i32,
    blur: u32,
    color: Option<HSLuv>,
}

impl Shadow {
    /// Create a new default shadow, which is not visible.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the shadow to appear inside the
    /// element on its top left side.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new()
    ///         .color(RED_3)
    ///         .inner()
    ///         .y(10)
    ///         .x(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn inner(mut self) -> Self {
        self.inner = true;
        self
    }

    /// Move the shadow left or right.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new().color(RED_3).inner().x(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    /// Move the shadow top or bottom.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new().color(RED_3).y(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    /// Increase the size of the shadow.
    /// Using a negative value will decrease
    /// it.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new().color(RED_3).spread(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn spread(mut self, spread: i32) -> Self {
        self.spread = spread;
        self
    }

    /// Add blur radius for the shadow.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new().color(RED_3).blur(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn blur(mut self, blur: u32) -> Self {
        self.blur = blur;
        self
    }

    /// Set the color for the shadow.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Shadows::new([Shadow::new()
    ///         .color(RED_3)
    ///         .inner()
    ///         .x(10)
    ///         .y(10)]))
    ///     .s(Width::new(50))
    ///     .s(Height::new(50))
    ///     .label("Click me");
    /// ```
    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.color = Some(color);
        }
        self
    }
}

impl<'a> IntoCowStr<'a> for Shadow {
    fn into_cow_str(self) -> Cow<'a, str> {
        let mut shadow_settings = Vec::<Cow<_>>::new();
        if self.inner {
            shadow_settings.push("inset".into())
        }
        shadow_settings.extend([px(self.x), px(self.y), px(self.blur), px(self.spread)]);
        if let Some(color) = self.color {
            shadow_settings.push(color.into_cow_str());
        }
        shadow_settings.join(" ").into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        unimplemented!()
    }
}
