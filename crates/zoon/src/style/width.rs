use crate::*;
use std::{cell::Cell, collections::BTreeMap};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(Default)]
pub struct Width<'a> {
    css_props: BTreeMap<CssName, Cell<Option<CssPropValue<'a>>>>,
    width_mode: WidthMode,
    self_signal: Option<Box<dyn Signal<Item = Option<Self>> + Unpin>>,
}

fn into_prop_value<'a>(value: impl IntoCowStr<'a>) -> Cell<Option<CssPropValue<'a>>> {
    Cell::new(Some(CssPropValue::new(value)))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
enum CssName {
    MinWidth,
    Width,
    MaxWidth,
    FlexGrow,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum WidthMode {
    #[default]
    ExactWidth,
    FillWidth,
}

impl<'a> Width<'a> {
    /// Define the width with pixels for an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::exact(50)).label("Click me");
    /// ```
    pub fn exact(width: u32) -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Width, into_prop_value(px(width)));
        this.width_mode = WidthMode::ExactWidth;
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
    ///     .s(Gap::both(20))
    ///     .s(Width::exact_signal(
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
    pub fn exact_signal(
        width: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        Self::with_signal(width.map(|width| width.into().map(Width::exact)))
    }

    pub fn with_signal(
        width: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let width = width.map(|width| width.into());
        this.self_signal = Some(Box::new(width));
        this
    }

    /// Define the width with the number of `0` characters with its current
    /// font. More information at <https://stackoverflow.com/questions/48649169/what-is-difference-between-css-em-and-ch-units>.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::zeros(4)).label("Click me");
    /// ```
    pub fn zeros(zeros: u32) -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Width, into_prop_value(ch(zeros)));
        this.width_mode = WidthMode::ExactWidth;
        this
    }

    pub fn percent(percent: impl Into<f64>) -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Width, into_prop_value(pct(percent.into())));
        this.width_mode = WidthMode::ExactWidth;
        this
    }

    pub fn percent_signal<T: Into<f64>>(
        width: impl Signal<Item = impl Into<Option<T>>> + Unpin + 'static,
    ) -> Self {
        Self::with_signal(width.map(|width| width.into().map(|width| Width::percent(width.into()))))
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
        this.css_props
            .insert(CssName::Width, into_prop_value("100%"));
        this.width_mode = WidthMode::FillWidth;
        this
    }

    pub fn growable() -> Self {
        Self::growable_with_factor::<f64>(None)
    }

    pub fn growable_with_factor<T: Into<f64>>(factor: impl Into<Option<T>>) -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Width, into_prop_value("auto"));
        if let Some(factor) = factor.into() {
            this.css_props
                .insert(CssName::FlexGrow, into_prop_value(factor.into()));
        }
        this.width_mode = WidthMode::FillWidth;
        this
    }

    /// Set the element minimum width.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let container = Column::new()
    ///     .s(Width::exact(5))
    ///     .item(Button::new().s(Width::fill().min(25)).label("Click me"));
    /// ```
    pub fn min(mut self, width: u32) -> Self {
        self.css_props
            .insert(CssName::MinWidth, into_prop_value(px(width)));
        self
    }

    /// Set the element maximum width.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Width::fill().max(25)).label("Click me");
    /// ```
    pub fn max(mut self, width: u32) -> Self {
        self.css_props
            .insert(CssName::MaxWidth, into_prop_value(px(width)));
        self
    }

    /// Set the maximum element width to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Width::fill().max_fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn max_fill(mut self) -> Self {
        self.css_props
            .insert(CssName::MaxWidth, into_prop_value("100%"));
        self
    }
}

impl<'a> Style<'a> for Width<'static> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self {
                css_props,
                width_mode,
                self_signal,
            } = self;

            if let Some(self_signal) = self_signal {
                let self_signal = self_signal.broadcast();

                for name in CssName::iter() {
                    group = group.style_signal(
                        <&str>::from(name),
                        self_signal.signal_ref(move |this| {
                            this.as_ref().and_then(|this| {
                                this.css_props
                                    .get(&name)
                                    .and_then(|value| value.take().map(|value| value.value))
                            })
                        }),
                    );
                }

                for mode in WidthMode::iter() {
                    group = group.class_signal(
                        <&str>::from(mode),
                        self_signal.signal_ref(move |this| {
                            this.as_ref()
                                .map(|this| this.width_mode == mode)
                                .unwrap_or_default()
                        }),
                    );
                }
                group
            } else {
                group.static_css_props.extend(
                    css_props
                        .into_iter()
                        .map(|(name, value)| (name.into(), value.take().unwrap_throw())),
                );
                group.class(width_mode.into())
            }
        });
    }
}
