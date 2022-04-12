use crate::*;

#[derive(Default)]
pub struct Width<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
    width_mode: WidthMode,
}

enum WidthMode {
    Exact,
    Fill,
}

impl Default for WidthMode {
    fn default() -> Self {
        Self::Exact
    }
}

impl<'a> Width<'a> {
    /// Define the width with pixels for an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::new(50)).label("Click ne");
    /// ```
    pub fn new(width: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", px(width));
        this.width_mode = WidthMode::Exact;
        this
    }

    /// Define the width with pixels depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let container_width = Mutable::new(0);
    /// let container_width_broadcaster = Broadcaster::new(container_width.signal());
    ///
    /// let button = Row::new()
    ///     .s(Align::center())
    ///     .s(Font::new().color(GRAY_8))
    ///     .s(Background::new().color(BLUE_4))
    ///     .s(Borders::all(Border::new().color(BLUE_8).width(3)))
    ///     .s(Spacing::new(20))
    ///     .s(Width::with_signal(
    ///         container_width_broadcaster.signal().map(|width| {
    ///             if width > 1000 {
    ///                 return width / 2;
    ///             }
    ///             (f64::from(width) * 0.9) as u32
    ///         }),
    ///     ))
    ///     .item("Change the window size!")
    ///     .item(
    ///         El::new()
    ///             .s(Align::new().right())
    ///             .child_signal(container_width_broadcaster.signal()),
    ///     );
    ///
    /// let container = El::new()
    ///     .s(Width::fill())
    ///     .s(Height::fill())
    ///     .on_viewport_size_change(move |width, _| container_width.set_neq(width))
    ///     .child(button);
    /// ```
    pub fn with_signal(
        width: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let width = width.map(|width| width.into().map(px));
        this.dynamic_css_props
            .insert("width".into(), box_css_signal(width));
        this.width_mode = WidthMode::Exact;
        this
    }

    /// Define the width with the number of `0` characters with its current
    /// font. More information at <https://stackoverflow.com/questions/48649169/what-is-difference-between-css-em-and-ch-units>.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::zeros(4)).label("Click ne");
    /// ```
    pub fn zeros(zeros: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", ch(zeros));
        this.width_mode = WidthMode::Exact;
        this
    }

    /// Set the element width to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Width::fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("width", "100%");
        this.width_mode = WidthMode::Fill;
        this
    }

    /// Set the element minimum width.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let container = Column::new()
    ///     .s(Width::new(5))
    ///     .item(Button::new().s(Width::default().min(25)).label("Click ne"));
    /// ```
    pub fn min(mut self, width: u32) -> Self {
        self.static_css_props.insert("min-width", px(width));
        self
    }

    /// Set the element maximum width.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::default().max(25)).label("Click ne");
    /// ```
    pub fn max(mut self, width: u32) -> Self {
        self.static_css_props.insert("max-width", px(width));
        self
    }

    /// Set the maximum element width to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Width::default().max_fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn max_fill(mut self) -> Self {
        self.static_css_props.insert("max-width", "100%");
        self
    }
}

impl<'a> Style<'a> for Width<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        let width_mode_class = match self.width_mode {
            WidthMode::Exact => "exact_width",
            WidthMode::Fill => "fill_width",
        };
        raw_el = raw_el.class(&width_mode_class);

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
