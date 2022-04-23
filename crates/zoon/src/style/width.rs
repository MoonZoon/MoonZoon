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

// @TODO derive `Default` for `WidthMode` and other enums once possible.
// https://rust-lang.github.io/rfcs/3107-derive-default-enum.html
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
    fn merge_with_group(self, mut group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            static_css_props,
            dynamic_css_props,
            width_mode,
        } = self;
        group.static_css_props.extend(static_css_props);
        group.dynamic_css_props.extend(dynamic_css_props);

        let width_mode_class = match width_mode {
            WidthMode::Exact => "exact_width",
            WidthMode::Fill => "fill_width",
        };
        group.class(width_mode_class)
    }
}
