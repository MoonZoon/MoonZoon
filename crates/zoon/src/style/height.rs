use crate::*;

/// Styling for height.
#[derive(Default)]
pub struct Height<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
    height_mode: HeightMode,
}

enum HeightMode {
    Exact,
    Fill,
}

// @TODO remove (in the entire codebase) once `derive_default_enum` is stable
// https://github.com/rust-lang/rust/issues/87517
impl Default for HeightMode {
    fn default() -> Self {
        Self::Exact
    }
}

impl<'a> Height<'a> {
    /// Define the height with pixels for an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Height::new(50)).label("Click me");
    /// ```
    pub fn new(height: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", px(height));
        this.height_mode = HeightMode::Exact;
        this
    }

    /// Define the height with pixels depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (is_hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Height::with_signal(hover_signal.map_bool(|| 50, || 100)))
    ///     .on_hovered_change(move |hover| is_hovered.set(hover))
    ///     .label("hover me");
    /// ```
    pub fn with_signal(
        height: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let height = height.map(|height| height.into().map(px));
        this.dynamic_css_props
            .insert("height".into(), box_css_signal(height));
        this.height_mode = HeightMode::Exact;
        this
    }

    /// Set the element height to fill its container or parent element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100%");
        this.height_mode = HeightMode::Fill;
        this
    }

    /// THe element height will be the height of thw device screen or web
    /// browser frame.
    /// #Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100vh");
        this.height_mode = HeightMode::Exact;
        this
    }

    /// THe element minimum height will be the height of thw device screen or
    /// web browser frame.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::default().min_screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn min_screen(mut self) -> Self {
        self.static_css_props.insert("min-height", "100vh");
        self
    }

    /// The element maximum height can be set by value in pixels.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::default().max(150))
    ///     .label("Hover this giant button");
    /// ```
    pub fn max(mut self, height: u32) -> Self {
        self.static_css_props.insert("max-height", px(height));
        self
    }

    /// Set the maximum element height to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::default().max_fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn max_fill(mut self) -> Self {
        self.static_css_props.insert("max-height", "100%");
        self
    }
}

impl<'a> Style<'a> for Height<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        let height_mode_class = match self.height_mode {
            HeightMode::Exact => "exact_height",
            HeightMode::Fill => "fill_height",
        };
        raw_el = raw_el.class(&height_mode_class);

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
