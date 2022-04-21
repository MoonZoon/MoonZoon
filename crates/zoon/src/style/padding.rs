use crate::*;

/// Define padding with pixels for an element.
#[derive(Default)]
pub struct Padding<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Padding<'a> {
    /// Set all paddings, e.g top, right, bottom and left for an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::all(15)).label("Click me");
    /// ```
    pub fn all(padding: u32) -> Self {
        Self::default().x(padding).y(padding)
    }

    /// Set all paddings, e.g top, right, bottom and left depending of signal's
    /// state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::all_signal(hover_signal.map_bool(|| 20, || 0)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn all_signal(all: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let this = Self::default();
        let all = Broadcaster::new(all.map(|all| all.into()));
        this.x_signal(all.signal()).y_signal(all.signal())
    }

    /// Set left and right padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().x(15)).label("Click me");
    /// ```
    pub fn x(self, x: u32) -> Self {
        self.left(x).right(x)
    }

    /// Set left and right padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().x_signal(hover_signal.map_bool(|| 20, || 0)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn x_signal(self, x: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let x = Broadcaster::new(x.map(|x| x.into()));
        self.left_signal(x.signal()).right_signal(x.signal())
    }

    /// Set top and bottom padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().y(15)).label("Click me");
    /// ```
    pub fn y(self, y: u32) -> Self {
        self.top(y).bottom(y)
    }

    /// Set top and bottom padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().y_signal(hover_signal.map_bool(|| 20, || 0)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn y_signal(self, y: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static) -> Self {
        let y = Broadcaster::new(y.map(|y| y.into()));
        self.top_signal(y.signal()).bottom_signal(y.signal())
    }

    /// Set top padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().top(15)).label("Click me");
    /// ```
    pub fn top(mut self, top: u32) -> Self {
        self.static_css_props.insert("padding-top", px(top));
        self
    }

    /// Set top padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().top_signal(hover_signal.map_bool(|| 20, || 0)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn top_signal(
        mut self,
        top: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let top = top.map(|top| top.into().map(px));
        self.dynamic_css_props
            .insert("padding-top".into(), box_css_signal(top));
        self
    }

    /// Set right padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().right(15)).label("Click me");
    /// ```
    pub fn right(mut self, right: u32) -> Self {
        self.static_css_props.insert("padding-right", px(right));
        self
    }

    /// Set right padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().right_signal(hover_signal.map_bool(|| 20, || 0)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn right_signal(
        mut self,
        right: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let right = right.map(|right| right.into().map(px));
        self.dynamic_css_props
            .insert("padding-right".into(), box_css_signal(right));
        self
    }

    /// Set bottom padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().bottom(15)).label("Click me");
    /// ```
    pub fn bottom(mut self, bottom: u32) -> Self {
        self.static_css_props.insert("padding-bottom", px(bottom));
        self
    }

    /// Set bottom padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().bottom_signal(hover_signal.map_true(|| 20)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn bottom_signal(
        mut self,
        bottom: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let bottom = bottom.map(|bottom| bottom.into().map(px));
        self.dynamic_css_props
            .insert("padding-bottom".into(), box_css_signal(bottom));
        self
    }

    /// Set left padding.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Padding::new().left(15)).label("Click me");
    /// ```
    pub fn left(mut self, left: u32) -> Self {
        self.static_css_props.insert("padding-left", px(left));
        self
    }

    /// Set left padding depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Padding::new().left_signal(hover_signal.map_true(|| 20)))
    ///     .on_hovered_change(move |hover| hovered.set(hover))
    ///     .label("Hover me");
    /// ```
    pub fn left_signal(
        mut self,
        left: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let left = left.map(|left| left.into().map(px));
        self.dynamic_css_props
            .insert("padding-left".into(), box_css_signal(left));
        self
    }
}

impl<'a> Style<'a> for Padding<'a> {
    fn merge_with_group(self, group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self { static_css_props, dynamic_css_props } = self;
        group.static_css_props.extend(static_css_props);
        group.dynamic_css_props.extend(dynamic_css_props);
        group
    }
}
