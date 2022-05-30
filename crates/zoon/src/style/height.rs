use crate::{style::supports_dvx, *};

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
    /// let button = Button::new().s(Height::exact(50)).label("Click me");
    /// ```
    pub fn exact(height: u32) -> Self {
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
    ///     .s(Height::exact_signal(hover_signal.map_bool(|| 50, || 100)))
    ///     .on_hovered_change(move |hover| is_hovered.set(hover))
    ///     .label("hover me");
    /// ```
    pub fn exact_signal(
        height: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let height = height.map(|height| height.into().map(px));
        this.dynamic_css_props
            .insert("height".into(), box_css_signal(height));
        this.height_mode = HeightMode::Exact;
        this
    }

    /// Set the element height to fill its container.
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

    /// The element height will be the height of the device screen or web
    /// browser frame.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("height", if *supports_dvx() { "100dvh" } else { "100vh" });
        this.height_mode = HeightMode::Exact;
        this
    }

    pub fn min(mut self, height: u32) -> Self {
        self.static_css_props.insert("min-height", px(height));
        self
    }

    /// The element minimum height will be the height of thw device screen or
    /// web browser frame.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill().min_screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn min_screen(mut self) -> Self {
        self.static_css_props.insert(
            "min-height",
            if *supports_dvx() { "100dvh" } else { "100vh" },
        );
        self
    }

    /// The element maximum height can be set by value in pixels.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill().max(150))
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
    ///     .s(Height::fill().max_fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn max_fill(mut self) -> Self {
        self.static_css_props.insert("max-height", "100%");
        self
    }
}

impl<'a> Style<'a> for Height<'a> {
    fn merge_with_group(self, mut group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            static_css_props,
            dynamic_css_props,
            height_mode,
        } = self;
        group.static_css_props.extend(static_css_props);
        group.dynamic_css_props.extend(dynamic_css_props);

        let height_mode_class = match height_mode {
            HeightMode::Exact => "exact_height",
            HeightMode::Fill => "fill_height",
        };
        group.class(height_mode_class)
    }
}
